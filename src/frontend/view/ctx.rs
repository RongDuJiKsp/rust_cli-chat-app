use crate::frontend::command::plainer::exec_command;
use crate::util::char::is_char_printable;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{cursor, execute, style};
use std::collections::VecDeque;
use std::io;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::sync::RwLock;
use crate::util::ctx::AppEventLoopContext;

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
        self.flush_screen_buffer().await?;
        self.write_buffer.write().await.clear();
        self.flush_input().await?;
        Ok(())
    }
    pub async fn user_backspace(&self) -> anyhow::Result<()> {
        self.write_buffer.write().await.pop();
        self.flush_input().await?;
        Ok(())
    }
    async fn flush_input(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = crossterm::terminal::size()?;
        let buf = &*self.write_buffer.read().await;
        let to_show_slice_from = if buf.len() < tem_w as usize { 0 } else { buf.len() - tem_w as usize };
        {
            let _permit = self.signal.acquire().await?;
            execute!(io::stdout(), cursor::MoveTo(0, tem_h - 1))?;
            execute!(io::stdout(), style::Print(" ".repeat(tem_w as usize)))?;
            execute!(io::stdout(), cursor::MoveTo(0, tem_h - 1))?;
            execute!(io::stdout(), style::Print(&buf[to_show_slice_from..]))?;
        }
        Ok(())
    }
    async fn flush_screen_buffer(&self) -> anyhow::Result<()> {
        //得到终端尺寸
        let (tem_w, tem_h) = crossterm::terminal::size()?;
        //缓冲区
        let mut screen_print_idx = tem_h as i32 - 3;
        let mut out_buf = Vec::with_capacity(screen_print_idx as usize);
        {
            //加锁，将缓存区内的字符串换行写入缓冲区
            let bufs = &*self.screen_buffer.read().await;
            for to_print in bufs.iter().rev() {
                let chars = to_print.chars().collect::<Vec<char>>();
                if screen_print_idx < 0 {
                    break;
                }
                for chuck in chars.chunks(tem_w as usize).rev() {
                    if screen_print_idx < 0 {
                        break;
                    }
                    out_buf.push(chuck.iter().collect::<String>());
                    screen_print_idx -= 1;
                }
            }
        }
        //输出缓冲区
        {
            let _permit = self.signal.acquire().await?;
            let mut stdout = io::stdout();
            execute!(stdout, cursor::SavePosition)?;
            for (local, chuck) in out_buf.into_iter().rev().enumerate() {
                execute!(stdout,cursor::MoveTo(0,local as u16))?;
                execute!(stdout,style::Print(chuck))?;
            }
            execute!(stdout, cursor::RestorePosition)?;
        }
        Ok(())
    }
}
pub async fn hd_terminal_event(app: &AppEventLoopContext, ctx: &mut PrinterCtx, screen_event: &Event) -> anyhow::Result<()> {
    //处理按键event
    if let Event::Key(key) = screen_event {
        //处理ctrl+c
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            println!("Ctrl+C pressed, exiting...");
            app.close().await;
            println!("Press any key to quit");
            return Ok(());
        }
        //只处理按下，不处理释放,防止重复导致的问题
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