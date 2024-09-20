use crate::entity::alias::sync::SharePtrFactory;
use crate::entity::dto::base_body::BaseSocketMessageBody;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub type LockedTcpStream = Arc<RwLock<TcpStream>>;
pub type AddrStreamMapping = Arc<RwLock<HashMap<SocketAddr, LockedTcpStream>>>;
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
    pub async fn send_raw(&self, addr: SocketAddr, end_point: String, raw: Option<String>) -> anyhow::Result<()> {
        let io = self.addr_stream(addr).await.ok_or(anyhow::anyhow!("No Conn on {}", addr))?;
        BaseSocketMessageBody::make_raw(end_point, raw).write_to(&*io.write().await)?;
        Ok(())
    }
    pub async fn server_stream(&self, addr: SocketAddr) -> Option<LockedTcpStream> {
        self.as_client.read().await.get(&addr).cloned()
    }
    pub async fn client_stream(&self, addr: SocketAddr) -> Option<LockedTcpStream> {
        self.as_server.read().await.get(&addr).cloned()
    }
    pub async fn addr_stream(&self, addr: SocketAddr) -> Option<LockedTcpStream> {
        match self.client_stream(addr).await {
            Some(s) => Some(s),
            None => match self.server_stream(addr).await {
                None => None,
                Some(s) => Some(s)
            }
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.this
    }
}
