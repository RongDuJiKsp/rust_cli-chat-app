use crate::backend::connect::ctx::ConnCtx;
use crate::backend::connect::event::{ConnChan, ConnectHandler};
use crate::frontend::view::ctx::{hd_terminal_event, PrinterCtx};
use crate::frontend::view::event::{PrintEventHandler, PrinterChan};
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

pub struct MainApplication {
    args: ApplicationArgs,
    listener_port: u16,
    printer: (PrinterCtx, PrinterChan),
    connector: (ConnCtx, ConnChan),
}
impl MainApplication {
    pub async fn init() -> anyhow::Result<MainApplication> {
        //read args
        let args = ApplicationArgs::parse();
        let listener_port = args.port.unwrap_or_else(|| 0);
        //init printer
        PrintEventHandler::init_screen()?;
        let printer = PrintEventHandler::run_ctx()?;
        //init conn
        let connector = ConnectHandler::bind(&format!("0.0.0.0:{}", listener_port)).await?;
        //ok
        Ok(MainApplication {
            args,
            listener_port,
            printer,
            connector,
        })
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (printer_ctx, printer_chan) = &mut self.printer;
        let (conn_ctx, conn_chan) = &mut self.connector;
        loop {
            select! {
                Some(screen_event) = printer_chan.recv() => {
                    hd_terminal_event(printer_ctx,&screen_event).await?;
                }
                Some(conn_event) = conn_chan.recv() => {}
            }
        }
    }
    pub async fn destroy(&mut self) -> anyhow::Result<()> {
        //destroy screen
        PrintEventHandler::delete_screen()?;
        Ok(())
    }
}
