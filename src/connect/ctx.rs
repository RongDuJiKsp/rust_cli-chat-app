use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
pub struct SocketCtx {
    addr: SocketAddr,
    stream: TcpStream,
}
#[derive(Clone)]
pub struct ConnCtx {
    connects: Arc<Mutex<Vec<SocketCtx>>>,
}
impl ConnCtx {
    pub fn new() -> ConnCtx {
        ConnCtx {
            connects: Arc::new(Mutex::new(Vec::new())),
        }
    }
}