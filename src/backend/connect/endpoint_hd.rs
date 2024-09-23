use crate::backend::chat::body::BaseChatMessageBody;
use crate::main_application::ApplicationLifetime;
use crate::util::log_fmt::LogFormatter;
use std::net::SocketAddr;

pub async fn hd_ep_msgbox(
    app: &ApplicationLifetime,
    addr: SocketAddr,
    raw: String,
) -> anyhow::Result<()> {
    app.printer
        .write_many(LogFormatter::info(&format!(
            "A msgbox from {} is : {}",
            addr, raw
        )))
        .await?;
    Ok(())
}
pub async fn hd_ep_chat(
    app: &ApplicationLifetime,
    that: &SocketAddr,
    raw: String,
) -> anyhow::Result<()> {
    app.chat
        .new_msg(BaseChatMessageBody {
            msg: raw,
            me: that.clone(),
        })
        .await?;
    if app.chat.is_chatting_with(that).await {
        app.chat.print_to(&app.printer).await?;
    }
    Ok(())
}
