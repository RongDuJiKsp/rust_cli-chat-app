use crate::entity::alias::sync::SharePtrFactory;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

type LockedTcpStream = Arc<RwLock<TcpStream>>;
type AddrStreamMapping = Arc<RwLock<HashMap<SocketAddr, LockedTcpStream>>>
#[derive(Clone)]
pub struct ConnCtx {
    this: SocketAddr,
    as_server: AddrStreamMapping,
    as_client: AddrStreamMapping,
}
impl ConnCtx {
    pub fn new(addr: SocketAddr) -> ConnCtx {
        ConnCtx {
            as_server: SharePtrFactory::new_shared_rw_ptr(HashMap::new()),
            as_client: SharePtrFactory::new_shared_rw_ptr(HashMap::new()),
            this: addr,
        }
    }
    pub async fn try_conn(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let conn = TcpStream::connect(addr).await?;
        self.as_client.write().await.insert(addr, SharePtrFactory::new_shared_rw_ptr(conn));
        Ok(())
    }
    pub async fn try_disconnect_server(&self, addr: SocketAddr) -> anyhow::Result<()> {
        self.as_client.write().await.remove(&addr).ok_or(anyhow::anyhow!("不存在的远端主机！"))?;
        Ok(())
    }
    pub fn addr(&self) -> SocketAddr {
        self.this
    }
}
