use crate::entity::alias::sync::{PtrFac, SharedRWPtr};
use crate::entity::dto::base_body::BaseSocketMessageBody;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use crate::backend::connect::event::ConnPointHd;
use crate::main_application::ApplicationLifetime;

pub type LockedTcpStream = SharedRWPtr<TcpStream>;
pub type AddrStreamMapping = SharedRWPtr<HashMap<SocketAddr, LockedTcpStream>>;
#[derive(Clone)]
pub struct ConnCtx {
    this: SocketAddr,
    as_server: AddrStreamMapping,
    as_client: AddrStreamMapping,
}
impl ConnCtx {
    pub fn new(addr: SocketAddr) -> ConnCtx {
        ConnCtx {
            as_server: PtrFac::shared_rw_ptr(HashMap::new()),
            as_client: PtrFac::shared_rw_ptr(HashMap::new()),
            this: addr,
        }
    }
    pub async fn add_client(&self, addr: SocketAddr, stream: TcpStream) {
        self.as_server
            .write()
            .await
            .insert(addr, PtrFac::shared_rw_ptr(stream));
    }
    pub async fn try_conn(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let conn = TcpStream::connect(addr).await?;
        self.as_client
            .write()
            .await
            .insert(addr, PtrFac::shared_rw_ptr(conn));
        Ok(())
    }
    pub async fn try_disconnect_server(&self, addr: SocketAddr) -> anyhow::Result<()> {
        self.as_client
            .write()
            .await
            .remove(&addr)
            .ok_or(anyhow::anyhow!("不存在的远端主机！"))?;
        Ok(())
    }
    pub async fn send_raw(
        &self,
        addr: SocketAddr,
        end_point: String,
        raw: Option<String>,
    ) -> anyhow::Result<()> {
        let io = self
            .addr_stream(addr)
            .await
            .ok_or(anyhow::anyhow!("No Conn on {}", addr))?;
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
                Some(s) => Some(s),
            },
        }
    }
    pub fn addr(&self) -> SocketAddr {
        self.this
    }
}
pub async fn hd_conn_event(app: &ApplicationLifetime, hd: ConnPointHd) -> anyhow::Result<()> {
    app.conn.add_client(hd.1, hd.0).await;
    Ok(())
}