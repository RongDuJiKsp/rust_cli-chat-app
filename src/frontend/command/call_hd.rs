use crate::main_application::ApplicationLifetime;
use std::net::SocketAddr;

type Ctx<'a> = &'a ApplicationLifetime;
pub async fn call_conn(ctx: Ctx<'_>, addr: SocketAddr) -> anyhow::Result<()> {
    ctx.conn.try_conn(addr).await?;
    Ok(())
}
pub async fn call_dis_conn(ctx: Ctx<'_>, addr: SocketAddr) -> anyhow::Result<()> {
    Ok(())
}
