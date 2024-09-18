use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
#[derive(Clone)]
pub struct SocketConnectCtx {
    addr: SocketAddr,
    stream: Arc<Mutex<TcpStream>>,
}
#[derive(Clone)]
pub struct ConnCtx {
    as_server: Arc<Mutex<Vec<SocketConnectCtx>>>,
    as_client: Arc<Mutex<Vec<SocketConnectCtx>>>,
}
impl ConnCtx {
    pub fn new() -> ConnCtx {
        ConnCtx {
            as_server: Arc::new(Mutex::new(Vec::new())),
            as_client: Arc::new(Mutex::new(Vec::new())),
        }
    }

}
