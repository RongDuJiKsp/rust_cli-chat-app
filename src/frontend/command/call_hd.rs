use crate::main_application::ApplicationLifetime;
use std::net::SocketAddr;

type Ctx<'a> = &'a ApplicationLifetime;
pub struct CmdCallHandler;
impl CmdCallHandler {
    pub async fn call_conn(ctx: Ctx<'_>, addr: SocketAddr) -> anyhow::Result<()> {
        ctx.conn.try_conn(addr).await?;
        Ok(())
    }
    pub async fn call_dis_conn(ctx: Ctx<'_>, addr: SocketAddr) -> anyhow::Result<()> {
        ctx.conn.try_disconnect_server(addr).await?;
        Ok(())
    }
    pub async fn call_unsafe_msgbox(ctx: Ctx<'_>, addr: SocketAddr, msg: String) -> anyhow::Result<()> {
        ctx.conn.send_raw(addr, "msgbox".to_string(), Some(msg)).await?;
        Ok(())
    }
}
