use crate::config::buffer_size::SCREEN_BUFFER_SIZE;
use crate::frontend::command::plainer::CommendPlainer;
use crate::frontend::command::status::CommandStatusCtx;
use crate::main_application::ApplicationLifetime;
use crate::util::char::is_char_printable;
use crate::util::log_fmt::LogFormatter;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{cursor, execute, style, terminal};
use std::collections::VecDeque;
use std::io;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct PrinterCtx {
    write_buffer: Arc<RwLock<String>>,
    screen_buffer: Arc<RwLock<VecDeque<String>>>,
    command_status_ctx: Arc<RwLock<CommandStatusCtx>>,
    stdout_lock: Arc<Semaphore>,
}
impl PrinterCtx {
    pub fn new() -> PrinterCtx {
        PrinterCtx {
            write_buffer: Arc::new(RwLock::new(String::new())),
            screen_buffer: Arc::new(RwLock::new(VecDeque::with_capacity(SCREEN_BUFFER_SIZE))),
            stdout_lock: Arc::new(Semaphore::new(1)),
            command_status_ctx: Arc::new(RwLock::new(CommandStatusCtx::new())),
        }
    }
    pub async fn write_many(&self, o: Vec<String>) -> anyhow::Result<()> {
        let mut buf = self.screen_buffer.write().await;
        for s in o {
            buf.push_back(s);
        }
        drop(buf);
        self.flush_screen_buffer().await?;
        Ok(())
    }
    pub async fn write_output(&self, o: String) -> anyhow::Result<()> {
        self.screen_buffer.write().await.push_back(o);
        self.flush_screen_buffer().await?;
        Ok(())
    }
    pub async fn flush_all(&self) -> anyhow::Result<()> {
        self.flush_screen_buffer().await?;
        self.flush_input().await?;
        self.flush_status().await?;
        Ok(())
    }
    pub async fn user_ascii_input(&self, input: char) -> anyhow::Result<()> {
        self.write_buffer.write().await.push(input);
        self.flush_input().await?;
        self.command_status_ctx.write().await.typed_alpha += 1;
        self.flush_status().await?;
        Ok(())
    }
    pub async fn user_conform(&self, app: &ApplicationLifetime) -> anyhow::Result<()> {
        //get command
        let mut user_input = self.write_buffer.write().await;
        let command = user_input.clone();
        user_input.clear();
        drop(user_input);
        let mut status_ref = self.command_status_ctx.write().await;
        status_ref.last_command = command.clone();
        status_ref.typed_command += 1;
        drop(status_ref);
        self.flush_all().await?;
        //run command
        let that_app = app.clone();
        tokio::spawn(async move {
            let exec_res = match CommendPlainer::load_app(that_app.clone())
                .exec_command(&command)
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    let _ = that_app
                        .printer
                        .write_many(LogFormatter::error(&format!(
                            "Command Execution failed: {}",
                            e
                        )))
                        .await;
                    return;
                }
            };
            let mut buf_writer = that_app.printer.screen_buffer.write().await;
            if exec_res.need_clear() {
                buf_writer.clear();
            }
            for output in exec_res.output().into_iter() {
                buf_writer.push_back(output);
            }
            drop(buf_writer);
            let _ = that_app.printer.flush_all().await;
        });
        Ok(())
    }
    pub async fn user_backspace(&self) -> anyhow::Result<()> {
        self.write_buffer.write().await.pop();
        self.flush_input().await?;
        self.flush_status().await?;
        Ok(())
    }
    async fn flush_status(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = terminal::size()?;
        let status = self.command_status_ctx.read().await.to_string();
        let lock_ref = Arc::clone(&self.stdout_lock);
        tokio::spawn(async move {
            let mut stdout = io::stdout();
            let _permit = lock_ref
                .acquire()
                .await
                .expect("Couldn't acquire stdout lock");
            execute!(stdout, cursor::SavePosition);
            execute!(stdout, cursor::MoveTo(0, tem_h - 2));
            execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine));
            execute!(
                stdout,
                style::Print(&status.to_string()[0..status.len().min(tem_w as usize)])
            );
            execute!(stdout, cursor::RestorePosition);
        });
        Ok(())
    }
    async fn flush_input(&self) -> anyhow::Result<()> {
        let (tem_w, tem_h) = terminal::size()?;
        let buf_ref = self.write_buffer.clone();
        let lock_ref = Arc::clone(&self.stdout_lock);
        let buf = buf_ref.read().await;
        let to_show_slice_from = if buf.len() < tem_w as usize {
            0
        } else {
            buf.len() - tem_w as usize
        };
        drop(buf);
        tokio::spawn(async move {
            let buf = buf_ref.read().await;
            let mut stdout = io::stdout();
            {
                let _permit = lock_ref
                    .acquire()
                    .await
                    .expect("Couldn't acquire stdout lock");
                execute!(stdout, cursor::MoveTo(0, tem_h - 1));
                execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine));
                execute!(stdout, style::Print(&buf[to_show_slice_from..]));
            }
        });
        Ok(())
    }
    async fn flush_screen_buffer(&self) -> anyhow::Result<()> {
        //得到终端尺寸
        let (tem_w, tem_h) = terminal::size()?;
        //缓冲区
        let mut screen_print_idx = tem_h as i32 - 3;
        let mut out_buf = Vec::with_capacity(screen_print_idx as usize);
        let screen_buf_ref = self.screen_buffer.clone();
        let lock_ref = Arc::clone(&self.stdout_lock);
        tokio::spawn(async move {
            //将缓存区内的字符串换行写入缓冲区
            let bufs = screen_buf_ref.read().await;
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
            drop(bufs);
            //输出缓冲区
            let mut stdout = io::stdout();
            {
                let _permit = lock_ref
                    .acquire()
                    .await
                    .expect("Couldn't acquire screen buffer");
                execute!(stdout, cursor::SavePosition);
                for (local, chuck) in out_buf.into_iter().rev().enumerate() {
                    execute!(stdout, cursor::MoveTo(0, local as u16));
                    execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine));
                    execute!(stdout, style::Print(chuck));
                }
                execute!(stdout, cursor::RestorePosition);
            }
        });

        Ok(())
    }
}
pub async fn hd_terminal_event(
    application: &mut ApplicationLifetime,
    screen_event: &Event,
) -> anyhow::Result<()> {
    let mut ctx = &application.printer;
    let mut app = &application.event_loop;
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
                ctx.user_conform(application).await?;
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
