use crate::config::buffer_size::{CONNECT_BUFFER_SIZE, READER_BUFFER_SIZE, READER_TIME_SIZE};
use crate::config::message::MESSAGE_SPLITTER;
use crate::entity::dto::base_body::BaseSocketMessageBody;
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct FrameBody {
    pub frame: BaseSocketMessageBody,
    pub addr: SocketAddr,
}
impl FrameBody {
    pub fn new(frame: BaseSocketMessageBody, addr: SocketAddr) -> Self {
        Self { frame, addr }
    }
}
pub type FrameChan = Receiver<FrameBody>;
pub struct FrameReaderManager {
    sender_ref: Sender<FrameBody>,
}
impl FrameReaderManager {
    pub fn init() -> (FrameReaderManager, FrameChan) {
        let (tx, rx) = mpsc::channel(CONNECT_BUFFER_SIZE);
        let mgr = FrameReaderManager { sender_ref: tx };
        (mgr, rx)
    }
    pub fn bind(&self, addr: SocketAddr, hd: OwnedReadHalf) {
        let rf = self.sender_ref.clone();
        let mut hd = hd;
        tokio::spawn(async move {
            let mut un_hd_buf = Vec::with_capacity(READER_BUFFER_SIZE);
            loop {
                let mut buffer = [0; READER_TIME_SIZE];
                let read_size = match hd.read(&mut buffer).await {
                    Ok(s) => s,
                    Err(_) => {
                        break;
                    }
                };
                if read_size == 0 {
                    break;
                }
                for bytes in buffer[0..read_size].iter().cloned().collect::<Vec<_>>() {
                    if bytes == MESSAGE_SPLITTER {
                        let body = match BaseSocketMessageBody::unmarshal(&un_hd_buf) {
                            Ok(b) => b,
                            Err(_) => {
                                break;
                            }
                        };
                        un_hd_buf.clear();
                        let _ = rf.send(FrameBody::new(body, addr)).await.is_ok();
                    } else {
                        un_hd_buf.push(bytes);
                    }
                }
            }
        });
    }
}
