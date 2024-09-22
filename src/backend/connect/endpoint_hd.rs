use std::net::SocketAddr;
use crate::main_application::ApplicationLifetime;
use crate::util::log_fmt::LogFormatter;

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
