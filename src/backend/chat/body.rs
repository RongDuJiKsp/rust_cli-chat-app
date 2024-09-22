use std::net::SocketAddr;

pub struct BaseChatMessageBody {
    pub me: SocketAddr,
    pub msg: String,
}