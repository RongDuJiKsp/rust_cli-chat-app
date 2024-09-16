use crate::config::buffer_size;
use crate::view::ctx::PrinterCtx;
use crossterm::event::Event;
use crossterm::{event, terminal};
use tokio::sync::mpsc::Receiver;
pub type PrinterChan = Receiver<Event>;
pub struct PrintEventHandler {}
impl PrintEventHandler {
    pub fn init_screen() -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        Ok(())
    }
    pub fn delete_screen() -> anyhow::Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn run_ctx() -> anyhow::Result<(PrinterCtx, PrinterChan)> {
        let (tx, rx) = tokio::sync::mpsc::channel(buffer_size::KEYWORD_BUFFER_SIZE);
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            loop {
                let e = event::read()?;
                tx.blocking_send(e)?;
            }
        });
        Ok((PrinterCtx::new(), rx))
    }
}
