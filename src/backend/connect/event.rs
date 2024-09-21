use crate::backend::connect::ctx::ConnCtx;
use crate::backend::connect::resp_frame_reader::{FrameChan, FrameReaderManager};
use crate::config::buffer_size;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync;
use tokio::sync::mpsc::Receiver;

pub(crate) type ConnPointHd = (TcpStream, SocketAddr);
pub type ConnChan = Receiver<ConnPointHd>;
pub struct ConnectHandler {}
impl ConnectHandler {
    pub async fn bind(addr: &str) -> anyhow::Result<(ConnCtx, ConnChan, FrameChan)> {
        let listener = TcpListener::bind(addr).await?;
        let addr = listener.local_addr()?;
        let (tx, rx) = sync::mpsc::channel(buffer_size::CONNECT_BUFFER_SIZE);
        tokio::spawn(async move {
            loop {
                if let Ok(hd) = listener.accept().await {
                    //TODO:logger
                    let _ = tx.send(hd).await;
                }
            }
        });
        let (mgr, frame_chan) = FrameReaderManager::init();
        Ok((ConnCtx::new(addr, mgr), rx, frame_chan))
    }
}
