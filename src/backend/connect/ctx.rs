use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct ConnCtx {
    this: SocketAddr,
    as_server: Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
    as_client: Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
}
impl ConnCtx {
    pub fn new(addr: SocketAddr) -> ConnCtx {
        ConnCtx {
            as_server: Arc::new(RwLock::new(HashMap::new())),
            as_client: Arc::new(RwLock::new(HashMap::new())),
            this: addr,
        }
    }
    pub async fn try_conn(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let conn = TcpStream::connect(addr).await?;
        self.as_client.write().await.insert(addr, conn);
        Ok(())
    }
    pub fn addr(&self) -> SocketAddr {
        self.this
    }
}
