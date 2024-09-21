use crate::backend::connect::event::ConnPointHd;
use crate::backend::connect::resp_frame_reader::FrameBody;
use crate::main_application::ApplicationLifetime;
use crate::util::log_fmt::LogFormatter;
use std::net::SocketAddr;

pub async fn hd_conn_event(app: &ApplicationLifetime, hd: ConnPointHd) -> anyhow::Result<()> {
    app.conn.add_client(hd.1, hd.0).await;
    Ok(())
}
pub async fn hd_message_event(app: &ApplicationLifetime, msg: FrameBody) -> anyhow::Result<()> {
    let endpoint = msg.frame.end_point.clone();
    match endpoint.as_str() {
        "msgbox" => {
            let FrameBody { addr, frame } = msg;
            hd_ep_msgbox(app, addr, frame.be_raw()).await?;
        }
        _ => {}
    }
    Ok(())
}
async fn hd_ep_msgbox(
    app: &ApplicationLifetime,
    addr: SocketAddr,
    raw: String,
) -> anyhow::Result<()> {
    app.printer
        .write_many(LogFormatter::info(&format!(
            "A msgbox from {} is :\n{}",
            addr, raw
        )))
        .await?;
    Ok(())
}
