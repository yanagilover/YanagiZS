use std::{
    future::Future,
    io::{Cursor, Read, Write},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};
use dashmap::DashMap;
use futures::future::BoxFuture;
use qwer::{OctData, ProtocolID, ProtocolStream};
use tracing::{debug, warn};

use crate::{ProtocolContext, ProtocolPoint};

use super::middleware::{marshal_middleware_list, unmarshal_middleware_list, MiddlewareModel};

pub struct RpcPtcContext {
    pub point: Arc<RpcPtcPoint>,
    pub protocol_id: u16,
    pub arg: Box<[u8]>,
    pub addr: SocketAddr,
    pub middleware_list: Vec<MiddlewareModel>,
    arg_uid: u64,
    remote_channel: u16,
}

impl RpcPtcContext {
    pub fn get_arg<Arg: OctData>(&self) -> Result<Arg, std::io::Error> {
        let mut r = Cursor::new(&self.arg);
        Arg::unmarshal_from(&mut r, 0)
    }

    pub async fn send_ptc<RpcArg: OctData + ProtocolID>(&self, arg: RpcArg) {
        self.point
            .send_rpc(
                self.addr,
                self.remote_channel,
                arg,
                0,
                Vec::with_capacity(0),
            )
            .await;
    }

    pub async fn send_ret<RpcRet: OctData>(&self, ret: RpcRet) {
        self.point.send_ret(self.addr, ret, self.arg_uid).await;
    }
}

pub trait RpcHandler: Send + Sync {
    fn call(&self, context: RpcPtcContext) -> BoxFuture<'static, ()>;
}

impl<T, F> RpcHandler for T
where
    T: Fn(RpcPtcContext) -> F + Send + Sync,
    F: Future<Output = ()> + 'static + Send + Sync,
{
    fn call(&self, context: RpcPtcContext) -> BoxFuture<'static, ()> {
        Box::pin(self(context))
    }
}

pub struct RpcPtcPoint {
    pub protocol_point: ProtocolPoint,
    rpc_handlers: DashMap<u16, Box<dyn RpcHandler>>,
}

pub struct RpcRawRet(Box<[u8]>);

impl RpcRawRet {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RpcCallError {
    #[error("remote protocol service didn't reply")]
    NoResponse,
    #[error("failed to decode RpcRet")]
    Decode(#[from] std::io::Error),
}

impl RpcPtcPoint {
    pub fn new(point: ProtocolPoint) -> Arc<Self> {
        let point = Arc::new(Self {
            protocol_point: point,
            rpc_handlers: DashMap::new(),
        });

        let pt_clone = point.clone();
        point.protocol_point.register_rpc_call(move |ctx| {
            let point = pt_clone.clone();
            tokio::spawn(async move {
                point.handle_protocol_context(ctx).await;
            });
        });
        point
    }

    async fn handle_protocol_context(self: &Arc<Self>, context: ProtocolContext) {
        if context.body.len() < 6 {
            return;
        }

        let mut r = Cursor::new(&context.body);
        let protocol_id = r.read_u16::<LE>().unwrap();
        let arg_size = r.read_u32::<BE>().unwrap() as usize;
        if arg_size + 6 > context.body.len() {
            return;
        }

        let mut arg = vec![0u8; arg_size];
        r.read_exact(&mut arg).unwrap();

        let Ok(middleware_list) = unmarshal_middleware_list(&mut ProtocolStream::new(&mut r))
        else {
            debug!("failed to decode middleware list");
            return;
        };

        if let Some(cb) = self.rpc_handlers.get(&protocol_id) {
            cb.call(RpcPtcContext {
                point: self.clone(),
                protocol_id,
                arg: arg.into_boxed_slice(),
                arg_uid: context.rpc_arg_uid,
                addr: context.session.remote_addr,
                middleware_list,
                remote_channel: context.remote_channel,
            })
            .await;
        } else {
            warn!("RpcPtc: no handler registered for {protocol_id}");
        }
    }

    pub async fn send_rpc<RpcArg>(
        &self,
        addr: SocketAddr,
        to_channel: u16,
        rpc_arg: RpcArg,
        arg_uid: u64,
        middleware_list: Vec<MiddlewareModel>,
    ) where
        RpcArg: OctData + ProtocolID,
    {
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        let protocol_id = rpc_arg.get_protocol_id();
        cursor.write_u16::<LE>(protocol_id).unwrap();

        let mut arg_buf = Vec::new();
        rpc_arg
            .marshal_to(&mut Cursor::new(&mut arg_buf), 0)
            .unwrap();
        cursor.write_u32::<BE>(arg_buf.len() as u32).unwrap();
        cursor.write_all(&arg_buf).unwrap();

        marshal_middleware_list(&mut ProtocolStream::new(&mut cursor), &middleware_list).unwrap();

        self.protocol_point
            .send_rpc(addr, buf.into_boxed_slice(), to_channel, arg_uid)
            .await;

        if arg_uid == 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    pub async fn send_ret<RpcRet>(&self, addr: SocketAddr, rpc_ret: RpcRet, arg_uid: u64)
    where
        RpcRet: OctData,
    {
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        rpc_ret.marshal_to(&mut cursor, 0).unwrap();

        self.protocol_point
            .send_rpc(addr, buf.into_boxed_slice(), 0, arg_uid)
            .await;
    }

    pub async fn call_rpc<RpcArg, RpcRet>(
        &self,
        addr: SocketAddr,
        rpc_arg: RpcArg,
        middleware_list: Vec<MiddlewareModel>,
        timeout: Duration,
    ) -> Result<RpcRet, RpcCallError>
    where
        RpcArg: OctData + ProtocolID,
        RpcRet: OctData,
    {
        let raw_ret = self
            .call_rpc_raw(addr, rpc_arg, middleware_list, timeout)
            .await
            .ok_or(RpcCallError::NoResponse)?;

        let mut r = Cursor::new(raw_ret.as_bytes());
        let ret = RpcRet::unmarshal_from(&mut r, 0)?;

        Ok(ret)
    }

    pub async fn call_rpc_raw<RpcArg>(
        &self,
        addr: SocketAddr,
        rpc_arg: RpcArg,
        middleware_list: Vec<MiddlewareModel>,
        timeout: Duration,
    ) -> Option<RpcRawRet>
    where
        RpcArg: OctData + ProtocolID,
    {
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        let protocol_id = rpc_arg.get_protocol_id();
        cursor.write_u16::<LE>(protocol_id).unwrap();

        let mut arg_buf = Vec::new();
        rpc_arg
            .marshal_to(&mut Cursor::new(&mut arg_buf), 0)
            .unwrap();
        cursor.write_u32::<BE>(arg_buf.len() as u32).unwrap();
        cursor.write_all(&arg_buf).unwrap();

        marshal_middleware_list(&mut ProtocolStream::new(&mut cursor), &middleware_list).unwrap();

        let ret_buf = self
            .protocol_point
            .call_rpc(addr, buf.into_boxed_slice(), timeout)
            .await?;

        Some(RpcRawRet(ret_buf))
    }

    pub fn register_rpc_recv<T: RpcHandler + 'static>(&self, protocol_id: u16, cb: T) {
        self.rpc_handlers.insert(protocol_id, Box::new(cb));
    }
}
