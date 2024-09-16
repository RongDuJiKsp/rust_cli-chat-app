use crossterm::{cursor, execute, queue, style};
use std::collections::VecDeque;
use std::sync::Arc;
use std::io;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct PrinterCtx {
    write_buffer: Arc<Mutex<String>>,
    screen_buffer: Arc<Mutex<VecDeque<String>>>,
    signal: Arc<Semaphore>,
}
impl PrinterCtx {
    pub fn new() -> PrinterCtx {
        PrinterCtx {
            write_buffer: Arc::new(Mutex::new(String::new())),
            screen_buffer: Arc::new(Mutex::new(VecDeque::new())),
            signal: Arc::new(Semaphore::new(1)), //flush lock
        }
    }
    pub async fn user_ascii_input(&self, input: char) -> anyhow::Result<()> {
        self.write_buffer.lock().await.push(input);
        self.flush_input().await?;
        Ok(())
    }
    async fn flush_input(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = crossterm::terminal::size()?;
        let permit = self.signal.acquire().await?;
        let buf = &*self.write_buffer.lock().await;
        execute!(io::stdout(), cursor::MoveTo(0,tem_h-1))?;
        execute!(io::stdout(), cursor::MoveTo(0,tem_w-1))?;
        execute!(io::stdout(), style::Print(buf))?;
        permit.forget();
        Ok(())
    }
}