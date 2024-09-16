use clap::Parser;
use tokio::select;
use crate::connect::ctx::ConnCtx;
use crate::connect::event::{ConnChan, ConnectHandler};
use crate::view::ctx::PrinterCtx;
use crate::view::event::{PrintEventHandler, PrinterChan};

#[derive(Parser)]
struct ApplicationArgs {
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,
    #[clap(short = 'b', long = "bind_ip")]
    bind_ip: Option<String>,
    #[clap(short = 'n', long = "nick")]
    nick: Option<String>,
    #[clap(short = "buf", long = "channel_size", default_value = 1024)]
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
        Ok(MainApplication { args, listener_port, printer, connector })
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (printer_ctx, printer_chan) = &mut self.printer;
        let (conn_ctx, conn_chan) = &mut self.connector;
        loop {
            select! {
                key_event = printer_chan.recv() => {

                },
                conn_event = conn_chan.recv() => {

                }
            }
        }
        Ok(())
    }
}
