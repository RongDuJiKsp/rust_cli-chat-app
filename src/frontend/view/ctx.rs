use crate::frontend::command::plainer::exec_command;
use crate::util::char::is_char_printable;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::{cursor, execute, style};
use std::collections::VecDeque;
use std::io;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct PrinterCtx {
    write_buffer: Arc<RwLock<String>>,
    screen_buffer: Arc<RwLock<VecDeque<String>>>,
    signal: Arc<Semaphore>,
}
impl PrinterCtx {
    pub fn new() -> PrinterCtx {
        PrinterCtx {
            write_buffer: Arc::new(RwLock::new(String::new())),
            screen_buffer: Arc::new(RwLock::new(VecDeque::new())),
            signal: Arc::new(Semaphore::new(1)), //flush lock
        }
    }
    pub async fn user_ascii_input(&self, input: char) -> anyhow::Result<()> {
        self.write_buffer.write().await.push(input);
        self.flush_input().await?;
        Ok(())
    }
    pub async fn user_conform(&self) -> anyhow::Result<()> {
        let mut out_buf = Vec::new();
        exec_command(&self.write_buffer.read().await, &mut out_buf).await?;
        let mut buf_writer = self.screen_buffer.write().await;
        for output in out_buf.into_iter() {
            buf_writer.push_back(output);
        }
        drop(buf_writer);
        self.flush_input().await?;
        self.write_buffer.write().await.clear();
        Ok(())
    }
    pub async fn user_backspace(&self) -> anyhow::Result<()> {
        self.write_buffer.write().await.pop();
        self.flush_input().await?;
        Ok(())
    }
    async fn flush_input(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = crossterm::terminal::size()?;
        let _permit = self.signal.acquire().await?;
        let buf = &*self.write_buffer.read().await;
        let to_show_slice_from = if buf.len() < tem_w as usize { 0 } else { buf.len() - tem_w as usize };
        execute!(io::stdout(), cursor::MoveTo(0, tem_h - 1))?;
        execute!(io::stdout(), style::Print(&buf[to_show_slice_from..]))?;
        Ok(())
    }
    async fn flush_screen_buffer(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = crossterm::terminal::size()?;
        let permit = self.signal.acquire().await?;
        let bufs = &*self.screen_buffer.read().await;
        let to_show_slice_from = if bufs.len() < (tem_h as usize - 2) { 0 } else { bufs.len() - (tem_h as usize - 2) };
        let mut stdout = io::stdout();
        execute!(stdout, cursor::SavePosition)?;
        for (idx, to_show) in bufs.iter().skip(to_show_slice_from).enumerate() {
            let i = idx - to_show_slice_from;
            execute!(stdout, cursor::MoveTo(0, i as u16))?;
            execute!(io::stdout(), style::Print(to_show))?;
        }
        execute!(stdout, cursor::RestorePosition)?;
        permit.forget();
        Ok(())
    }
}
pub async fn hd_terminal_event(ctx: &mut PrinterCtx, screen_event: &Event) -> anyhow::Result<()> {
    //处理按键event
    if let Event::Key(key) = screen_event {
        //只处理按下，不处理释放
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }
        match key.code {
            KeyCode::Enter => {
                ctx.user_conform().await?;
            }
            KeyCode::Char(c) if is_char_printable(c) => {
                ctx.user_ascii_input(c).await?;
            }
            KeyCode::Backspace => {
                ctx.user_backspace().await?;
            }
            _ => {}
        }
    }
    //TODO:处理其他event
    Ok(())
}