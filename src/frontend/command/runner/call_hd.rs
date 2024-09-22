use crate::main_application::ApplicationLifetime;
use std::net::SocketAddr;
use serde_json::to_string;

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
    pub async fn call_unsafe_msgbox(
        ctx: Ctx<'_>,
        addr: SocketAddr,
        msg: String,
    ) -> anyhow::Result<()> {
        ctx.conn
            .send_raw(addr, "msgbox".to_string(), Some(msg))
            .await?;
        Ok(())
    }
    pub async fn call_conn_status(ctx: Ctx<'_>) -> anyhow::Result<Vec<String>> {
        let mut io_buf = Vec::new();
        let (liv, dead) = ctx.conn.conn_status().await;
        io_buf.push("-----------------系统状态-------------------".to_string());
        io_buf.push(format!("应用程序监听于：{}", ctx.conn.addr()));
        io_buf.push("-----------------存活连接-------------------".to_string());
        if liv.is_empty() {
            io_buf.push("无连接...".to_string());
        }
        for l in liv {
            io_buf.push(l.to_string())
        }
        io_buf.push("-----------------暂离连接-------------------".to_string());
        if dead.is_empty() {
            io_buf.push("无连接...".to_string());
        }
        for l in dead {
            io_buf.push(l.to_string())
        }
        Ok(io_buf)
    }
}
