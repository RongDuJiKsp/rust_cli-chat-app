use crate::backend::connect::ctx::ConnCtx;
use crate::backend::connect::event::{ConnChan, ConnectHandler};
use crate::frontend::view::ctx::{hd_terminal_event, PrinterCtx};
use crate::frontend::view::event::{PrintEventHandler, PrinterChan};
use crate::util::event_loop::AppEventLoopContext;
use clap::Parser;
use tokio::select;

#[derive(Parser)]
struct ApplicationArgs {
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,
    #[clap(short = 'b', long = "bind_ip")]
    bind_ip: Option<String>,
    #[clap(short = 'n', long = "nick")]
    nick: Option<String>,
    #[clap(short = 'f', long = "channel_size", default_value = "1024")]
    channel_size: usize,
}
#[derive(Clone)]
pub struct ApplicationLifetime {
    pub printer: PrinterCtx,
    pub conn: ConnCtx,
    pub event_loop: AppEventLoopContext,
}
pub struct ChannelUnions {
    printer: PrinterChan,
    conn: ConnChan,
}
pub struct MainApplication {
    args: ApplicationArgs,
    listener_port: u16,
    ctx: ApplicationLifetime,
    channel_unions: ChannelUnions,
}
impl MainApplication {
    pub async fn init() -> anyhow::Result<MainApplication> {
        //read args
        let args = ApplicationArgs::parse();
        let listener_port = args.port.unwrap_or_else(|| 0);
        //init printer
        PrintEventHandler::init_screen()?;
        let (printer_ctx, printer_chan) = PrintEventHandler::run_ctx()?;
        printer_ctx.flush_all().await?;
        //init conn
        let (conn_ctx, conn_chan) =
            ConnectHandler::bind(&format!("0.0.0.0:{}", listener_port)).await?;
        //init loop_ctx
        let event_loop_ctx = AppEventLoopContext::init();
        //ok
        let ctx = ApplicationLifetime {
            printer: printer_ctx,
            conn: conn_ctx,
            event_loop: event_loop_ctx,
        };
        let channel_unions = ChannelUnions {
            printer: printer_chan,
            conn: conn_chan,
        };
        Ok(MainApplication {
            args,
            listener_port,
            ctx,
            channel_unions,
        })
    }
    pub async fn defer_finally() {
        println!("程序正在清理...");
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        while *self.ctx.event_loop.event_looping.read().await {
            select! {
                Some(screen_event) = self.channel_unions.printer.recv() => {
                    hd_terminal_event(&mut self.ctx,&screen_event).await?;
                }
                Some(conn_event) = self.channel_unions.conn.recv() => {}
            }
        }
        Ok(())
    }
    pub async fn destroy(&mut self) -> anyhow::Result<()> {
        //destroy screen
        PrintEventHandler::delete_screen()?;
        Ok(())
    }
}
