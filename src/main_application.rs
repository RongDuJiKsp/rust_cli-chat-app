use crate::backend::chat::ctx::ChatCtx;
use crate::backend::connect::ctx::ConnCtx;
use crate::backend::connect::event::{ConnChan, ConnectHandler};
use crate::backend::connect::event_hd::{hd_conn_event, hd_message_event};
use crate::backend::connect::resp_frame_reader::FrameChan;
use crate::frontend::view::ctx::PrinterCtx;
use crate::frontend::view::event::{PrintEventHandler, PrinterChan};
use crate::frontend::view::event_hd::hd_terminal_event;
use crate::util::event_loop::AppEventLoopContext;
use crate::util::log_fmt::LogFormatter;
use clap::Parser;
use tokio::select;

#[derive(Parser)]
struct ApplicationArgs {
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,
    #[clap(short = 'b', long = "bind_ip", default_value = "127.0.0.1")]
    bind_ip: String,
    #[clap(short = 'n', long = "nick")]
    nick: Option<String>,
    #[clap(short = 'f', long = "channel_size", default_value = "1024")]
    channel_size: usize,
}
#[derive(Clone)]
pub struct ApplicationLifetime {
    pub printer: PrinterCtx,
    pub conn: ConnCtx,
    pub chat: ChatCtx,
    pub event_loop: AppEventLoopContext,
}
pub struct ChannelUnions {
    printer: PrinterChan,
    conn: ConnChan,
    frame: FrameChan,
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
        let ip = args.bind_ip.clone();
        //init printer
        PrintEventHandler::init_screen()?;
        let (printer_ctx, printer_chan) = PrintEventHandler::run_ctx()?;
        printer_ctx.flush_all().await?;
        printer_ctx
            .write_many(LogFormatter::info("Screen Init"))
            .await?;
        //init conn
        let (conn_ctx, conn_chan, frame_chan) =
            ConnectHandler::bind(&format!("{}:{}", ip, listener_port)).await?;
        printer_ctx
            .write_many(LogFormatter::info(&format!(
                "TCP Listener is  bind on {}",
                conn_ctx.addr()
            )))
            .await?;
        // init chat
        let chat_ctx = ChatCtx::new();
        //init loop_ctx
        let event_loop_ctx = AppEventLoopContext::init();
        //ok
        printer_ctx
            .write_many(LogFormatter::info("Application Init Successful"))
            .await?;
        let ctx = ApplicationLifetime {
            printer: printer_ctx,
            conn: conn_ctx,
            chat: chat_ctx,
            event_loop: event_loop_ctx,
        };
        let channel_unions = ChannelUnions {
            printer: printer_chan,
            conn: conn_chan,
            frame: frame_chan,
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
                    hd_terminal_event(&mut self.ctx,screen_event).await?;
                }
                Some(conn_event) = self.channel_unions.conn.recv() => {
                    hd_conn_event(&mut self.ctx,conn_event).await?
                }
                Some(fram_event)=self.channel_unions.frame.recv()=>{
                    hd_message_event(&mut self.ctx,fram_event).await?;
                }
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
