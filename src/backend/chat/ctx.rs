use crate::backend::connect::ctx::ConnCtx;
use crate::entity::alias::sync::{PtrFac, SharedPtr, SharedRWPtr};
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct ChatCtx {
    chatting: SharedPtr<Option<SocketAddr>>,
    history_char: SharedRWPtr<HashMap<SocketAddr, SharedRWPtr<Vec<String>>>>,
}
impl ChatCtx {
    pub fn new() -> ChatCtx {
        ChatCtx {
            chatting: PtrFac::shared_ptr(None),
            history_char: PtrFac::shared_rw_ptr(HashMap::new()),
        }
    }
    pub async fn change_chat(&self, chat: &SocketAddr) {
        let now = self.chatting.lock().await;
        if now.is_some() && now.unwrap() == *chat {
            return;
        }
        *now = Some(*chat);
        drop(now);
        if !self.history_char.read().await.contains_key(chat) {
            self.init_chat(chat).await;
        }
    }
    pub async fn send_msg(&self, ctx: &ConnCtx, msg: String) -> anyhow::Result<()> {
        if let Some(c) = &*self.chatting.lock().await {
            self.history_char
                .read()
                .await
                .get(c)
                .ok_or(anyhow::anyhow!("chat not init"))?
                .write()
                .await
                .push(msg.clone())
        } else {
            anyhow::bail!("No chat selected")
        }
        Ok(())
    }
    async fn init_chat(&self, addr: &SocketAddr) {
        let mut body = Vec::new();
        body.push(format!("--------Chat between {} ----", addr));
        self.history_char
            .write()
            .await
            .insert(addr.clone(), PtrFac::shared_rw_ptr(body));
    }
}
