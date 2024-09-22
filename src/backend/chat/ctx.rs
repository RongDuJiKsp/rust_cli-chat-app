use crate::backend::chat::body::BaseChatMessageBody;
use crate::backend::connect::ctx::ConnCtx;
use crate::entity::alias::sync::{PtrFac, SharedPtr, SharedRWPtr};
use crate::frontend::view::ctx::PrinterCtx;
use anyhow::bail;
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
        let mut now = self.chatting.lock().await;
        if now.is_some() && now.unwrap() == *chat {
            return;
        }
        *now = Some(*chat);
        drop(now);
        if !self.history_char.read().await.contains_key(chat) {
            self.init_chat(chat).await;
        }
    }
    pub async fn print_to(&self, ctx: &PrinterCtx) -> anyhow::Result<()> {
        let chatting = self.chatting.lock().await;
        let tg = match *chatting {
            None => {
                bail!("No Chatting")
            }
            Some(e) => e,
        };
        let output = self
            .history_char
            .read()
            .await
            .get(&tg)
            .ok_or(anyhow::anyhow!("No init chat"))?
            .read()
            .await
            .clone();
        ctx.write_with_task()
            .with_many(output)
            .with_cls()
            .run()
            .await;
        ctx.flush_all().await?;
        Ok(())
    }
    pub async fn send_msg(&self, ctx: &ConnCtx, msg: String) -> anyhow::Result<()> {
        if let Some(c) = &*self.chatting.lock().await {
            Self::send(ctx, c, msg.clone()).await?;
            self.history_char
                .read()
                .await
                .get(c)
                .ok_or(anyhow::anyhow!("chat not init"))?
                .write()
                .await
                .push(format!("[From You] {}", msg))
        } else {
            bail!("No chat selected")
        }
        Ok(())
    }
    pub async fn new_msg(&self, msg: BaseChatMessageBody) -> anyhow::Result<()> {
        if !self.history_char.read().await.contains_key(&msg.me) {
            self.init_chat(&msg.me).await;
        }
        self.history_char
            .read()
            .await
            .get(&msg.me)
            .ok_or(anyhow::anyhow!("chat not init"))?
            .write()
            .await
            .push(format!("[To You] {}", msg.msg.clone()));
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
    async fn send(conn: &ConnCtx, addr: &SocketAddr, msg: String) -> anyhow::Result<()> {
        conn.send_raw(
            addr.clone(),
            "chat".to_string(),
            Some(
                BaseChatMessageBody {
                    msg,
                    me: conn.addr(),
                }
                    .to_json()?,
            ),
        )
            .await?;
        Ok(())
    }
}
