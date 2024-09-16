use crossterm::event;
use crossterm::event::Event;
use tokio::sync::mpsc::Receiver;
use crate::config::buffer_size;
use crate::view::ctx::PrinterCtx;
pub type PrinterChan = Receiver<Event>;
pub struct PrintEventHandler {}
impl PrintEventHandler {
    pub fn init_screen() -> anyhow::Result<()> {
        Ok(())
    }
    pub fn run_ctx() -> anyhow::Result<(PrinterCtx, PrinterChan)> {
        let (tx, rx) = tokio::sync::mpsc::channel(buffer_size::KEYWORD_BUFFER_SIZE);
        tokio::task::spawn_blocking(move || {
            loop {
                let e = event::read()?;
                tx.blocking_send(e)?;
            }
        });
        Ok((PrinterCtx::new(), rx))
    }
}