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
    connects: Arc<Mutex<Vec<SocketConnectCtx>>>,
}
impl ConnCtx {
    pub fn new() -> ConnCtx {
        ConnCtx {
            connects: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
