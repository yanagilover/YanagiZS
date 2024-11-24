use std::io::Result;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::Mutex,
};

pub struct ProtocolListener {
    socket: TcpListener,
}

pub struct ProtocolLinker {
    pub remote_addr: SocketAddr,
    write_half: Mutex<OwnedWriteHalf>,
    read_half: Mutex<OwnedReadHalf>,
}

pub async fn listen(local_addr: SocketAddr) -> Result<ProtocolListener> {
    Ok(ProtocolListener {
        socket: TcpListener::bind(local_addr).await?,
    })
}

pub async fn connect(remote_addr: SocketAddr) -> Result<ProtocolLinker> {
    let stream = TcpStream::connect(remote_addr).await?;
    Ok(ProtocolLinker::new(remote_addr, stream))
}

impl ProtocolLinker {
    pub async fn send(&self, buf: &[u8]) {
        let _ = self.write_half.lock().await.write_all(buf).await;
    }

    pub async fn recv_some(&self, buf: &mut [u8]) -> Result<usize> {
        self.read_half.lock().await.read(buf).await
    }

    fn new(remote_addr: SocketAddr, stream: TcpStream) -> Self {
        let (read, write) = stream.into_split();

        ProtocolLinker {
            remote_addr,
            write_half: Mutex::new(write),
            read_half: Mutex::new(read),
        }
    }
}

impl ProtocolListener {
    pub async fn accept(&self) -> Result<ProtocolLinker> {
        let (stream, remote_addr) = self.socket.accept().await?;
        Ok(ProtocolLinker::new(remote_addr, stream))
    }
}
