use crate::config::buffer_size;
use crate::connect::ctx::ConnCtx;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync;
use tokio::sync::mpsc::Receiver;

type ConnPointHd = (TcpStream, SocketAddr);
pub type ConnChan = Receiver<ConnPointHd>;
pub struct ConnectHandler {}
impl ConnectHandler {
    pub async fn bind(addr: &str) -> anyhow::Result<(ConnCtx, ConnChan)> {
        let listener = TcpListener::bind(addr).await?;
        let (tx, rx) = sync::mpsc::channel(buffer_size::CONNECT_BUFFER_SIZE);
        tokio::spawn(async move {
            loop {
                if let Ok(hd) = listener.accept().await {
                    //TODO:logger
                    let _ = tx.send(hd).await;
                }
            }
        });
        Ok((ConnCtx::new(), rx))
    }
}