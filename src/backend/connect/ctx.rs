use crate::backend::connect::resp_frame_reader::FrameReaderManager;
use crate::entity::alias::sync::{PtrFac, SharedPtr, SharedRWPtr};
use crate::entity::dto::base_body::BaseSocketMessageBody;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use crate::backend::connect::alias::MetaSocketAddr;
use crate::config::message::MESSAGE_SPLITTER;

pub type LivingConn = Vec<MetaSocketAddr>;
pub type DeadConn = Vec<MetaSocketAddr>;
pub type OnlyWriteLockedTcpStream = SharedPtr<OwnedWriteHalf>;
#[derive(Clone)]
pub struct ConnCtx {
    this: SocketAddr,
    as_server: SharedRWPtr<HashMap<SocketAddr, OnlyWriteLockedTcpStream>>,
    as_client: SharedRWPtr<HashMap<SocketAddr, OnlyWriteLockedTcpStream>>,
    frame: Arc<FrameReaderManager>,
}
impl ConnCtx {
    pub fn new(addr: SocketAddr, frame: FrameReaderManager) -> ConnCtx {
        ConnCtx {
            as_server: PtrFac::shared_rw_ptr(HashMap::new()),
            as_client: PtrFac::shared_rw_ptr(HashMap::new()),
            this: addr,
            frame: Arc::new(frame),
        }
    }
    pub async fn add_client(&self, addr: SocketAddr, stream: TcpStream) {
        let (r_half, w_half) = stream.into_split();
        self.frame.bind(addr, r_half);
        self.as_server
            .write()
            .await
            .insert(addr, PtrFac::shared_ptr(w_half));
    }
    pub async fn try_conn(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let conn = TcpStream::connect(addr).await?;
        let (r_half, w_half) = conn.into_split();
        self.frame.bind(addr, r_half);
        self.as_client
            .write()
            .await
            .insert(addr, PtrFac::shared_ptr(w_half));
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
        let locks = self
            .addr_stream(addr)
            .await
            .ok_or(anyhow::anyhow!("No Conn on {}", addr))?;
        let mut io = locks.lock().await;
        BaseSocketMessageBody::make_raw(end_point, raw)
            .write_to(&mut *io)
            .await?;
        Ok(())
    }
    pub async fn server_stream(&self, addr: SocketAddr) -> Option<OnlyWriteLockedTcpStream> {
        self.as_client.read().await.get(&addr).cloned()
    }
    pub async fn client_stream(&self, addr: SocketAddr) -> Option<OnlyWriteLockedTcpStream> {
        self.as_server.read().await.get(&addr).cloned()
    }
    pub async fn addr_stream(&self, addr: SocketAddr) -> Option<OnlyWriteLockedTcpStream> {
        match self.client_stream(addr).await {
            Some(s) => Some(s),
            None => match self.server_stream(addr).await {
                None => None,
                Some(s) => Some(s),
            },
        }
    }
    pub async fn conn_status(&self) -> (LivingConn, DeadConn) {
        let (mut liv, mut dead) = (Vec::new(), Vec::new());
        for (addr, conn) in &*self.as_server.read().await {
            match conn.lock().await.write(&[MESSAGE_SPLITTER]).await {
                Ok(_) => liv.push(MetaSocketAddr::pkg(addr.clone(), "(Conn to Client)".to_string())),
                Err(_) => dead.push(MetaSocketAddr::pkg(addr.clone(), "(Conn to Client)".to_string()))
            }
        };
        for (addr, conn) in &*self.as_client.read().await {
            match conn.lock().await.write(&[MESSAGE_SPLITTER]).await {
                Ok(_) => liv.push(MetaSocketAddr::pkg(addr.clone(), "(Conn to Server)".to_string())),
                Err(_) => dead.push(MetaSocketAddr::pkg(addr.clone(), "(Conn to Server)".to_string()))
            }
        };
        (liv, dead)
    }
    pub fn addr(&self) -> SocketAddr {
        self.this
    }
}
