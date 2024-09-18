use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ConnCtx {
    as_server: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
    as_client: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
}
impl ConnCtx {
    pub fn new() -> ConnCtx {
        ConnCtx {
            as_server: Arc::new(Mutex::new(HashMap::new())),
            as_client: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn try_conn(&self, addr: &str) -> anyhow::Result<()> {
        let remote = SocketAddr::from_str(addr)?;
        let conn = net::TcpStream::connect(&remote).await?;
        self.as_client.lock().await.insert(remote, conn);
        Ok(())
    }
}
