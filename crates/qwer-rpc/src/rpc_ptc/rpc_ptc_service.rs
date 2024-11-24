use std::{net::SocketAddr, sync::Arc};

use crate::ProtocolServiceFrontend;

use super::rpc_ptc_point::RpcPtcPoint;

pub struct RpcPtcServiceFrontend {
    pub protocol_service_frontend: ProtocolServiceFrontend,
}

impl RpcPtcServiceFrontend {
    pub fn new(protocol_service: ProtocolServiceFrontend) -> Self {
        Self {
            protocol_service_frontend: protocol_service,
        }
    }

    pub async fn create_point(
        &self,
        local_addr: Option<SocketAddr>,
    ) -> Result<Arc<RpcPtcPoint>, std::io::Error> {
        let protocol_point = self
            .protocol_service_frontend
            .create_point(local_addr)
            .await?;
        Ok(RpcPtcPoint::new(protocol_point))
    }
}
