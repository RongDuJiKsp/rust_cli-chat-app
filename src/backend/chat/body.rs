use std::net::SocketAddr;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BaseChatMessageBody {
    pub me: SocketAddr,
    pub msg: String,
}
impl BaseChatMessageBody {
    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        serde_json::from_str(&s).map_err(|e| anyhow!("{}",e))
    }
    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).map_err(|e| anyhow!("{}",e))
    }
}