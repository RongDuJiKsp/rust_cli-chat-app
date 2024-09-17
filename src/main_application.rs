use crate::backend::connect::ctx::ConnCtx;
use crate::backend::connect::event::{ConnChan, ConnectHandler};
use crate::frontend::view::ctx::{hd_terminal_event, PrinterCtx};
use crate::frontend::view::event::{PrintEventHandler, PrinterChan};
use clap::Parser;
use tokio::{select};
use crate::util::event_loop::AppEventLoopContext;

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

pub struct MainApplication {
    args: ApplicationArgs,
    listener_port: u16,
    printer: (PrinterCtx, PrinterChan),
    connector: (ConnCtx, ConnChan),
    event_loop_ctx: AppEventLoopContext,
}
impl MainApplication {
    pub async fn init() -> anyhow::Result<MainApplication> {
        //read args
        let args = ApplicationArgs::parse();
        let listener_port = args.port.unwrap_or_else(|| 0);
        //init printer
        PrintEventHandler::init_screen()?;
        let printer = PrintEventHandler::run_ctx()?;
        printer.0.flush_all().await?;
        //init conn
        let connector = ConnectHandler::bind(&format!("0.0.0.0:{}", listener_port)).await?;
        //init loop_ctx
        let event_loop_ctx = AppEventLoopContext::init();
        //ok
        Ok(MainApplication {
            args,
            listener_port,
            printer,
            connector,
            event_loop_ctx,
        })
    }
    pub async fn defer_finally() {
        println!("程序正在清理...");
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (printer_ctx, printer_chan) = &mut self.printer;
        let (conn_ctx, conn_chan) = &mut self.connector;
        while *self.event_loop_ctx.event_looping.read().await {
            select! {
                Some(screen_event) = printer_chan.recv() => {
                    hd_terminal_event(&self.event_loop_ctx,printer_ctx,&screen_event).await?;
                }
                Some(conn_event) = conn_chan.recv() => {}
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
