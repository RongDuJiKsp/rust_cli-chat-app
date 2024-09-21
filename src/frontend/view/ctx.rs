use crate::config::buffer_size::SCREEN_BUFFER_SIZE;
use crate::config::style::CANT_PRINT_RANGE_HEIGHT;
use crate::entity::alias::sync::{PtrFac, SharedPtr, SharedRWPtr};
use crate::frontend::command::plainer::CommendPlainer;
use crate::frontend::command::status::CommandStatus;
use crate::main_application::ApplicationLifetime;
use crate::util::history_loader::HistoryLoader;
use crate::util::log_fmt::LogFormatter;
use anyhow::anyhow;
use crossterm::{cursor, execute, style, terminal};
use std::collections::VecDeque;
use std::io;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct PrinterCtx {
    write_buffer: SharedRWPtr<String>,
    screen_buffer: SharedRWPtr<VecDeque<String>>,
    screen_view: SharedRWPtr<u16>,
    command_status_ctx: SharedRWPtr<CommandStatus>,
    command_history: SharedPtr<HistoryLoader<String>>,
    stdout_lock: Arc<Semaphore>,
}
impl PrinterCtx {
    pub fn new() -> PrinterCtx {
        PrinterCtx {
            write_buffer: PtrFac::shared_rw_ptr(String::new()),
            screen_buffer: PtrFac::shared_rw_ptr(VecDeque::with_capacity(SCREEN_BUFFER_SIZE)),
            screen_view: PtrFac::shared_rw_ptr(0u16),
            command_status_ctx: PtrFac::shared_rw_ptr(CommandStatus::new()),
            command_history: PtrFac::shared_ptr(HistoryLoader::new()),
            stdout_lock: Arc::new(Semaphore::new(1)),
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
    pub async fn user_cmd_history(&self, off: i8) -> anyhow::Result<()> {
        let his_cmd = match off {
            1 => match self.command_history.lock().await.easily() {
                None => { return Ok(()) }
                Some(cmd) => cmd
            },
            -1 => match self.command_history.lock().await.later() {
                None => { return Ok(()) }
                Some(cmd) => cmd
            },
            _ => { return Err(anyhow::anyhow!("Op must be 1 or -1")) }
        };
        *self.write_buffer.write().await = his_cmd;
        self.flush_input().await?;
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
        self.command_history.lock().await.add(command.clone());
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
    pub async fn user_view_offset_changed(&self, off: i16) -> anyhow::Result<()> {
        let mut v = self.screen_view.write().await;
        let rs = match off {
            1 => {
                *v += 1;
                Ok(())
            }
            -1 => {
                if *v > 0 {
                    *v -= 1;
                }
                Ok(())
            }
            _ => Err(anyhow!("Why offset more than 1")),
        };
        drop(v);
        if rs.is_ok() {
            self.flush_screen_buffer().await?;
        }
        rs
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
        //当终端尺寸大于CANT_PRINT_RANGE_HEIGHT时才需要打印
        if tem_h <= CANT_PRINT_RANGE_HEIGHT {
            return Ok(());
        }
        let can_print = tem_h - CANT_PRINT_RANGE_HEIGHT;
        //得到视图偏移
        let offset_view = *self.screen_view.read().await;
        //计算需要打印的行数
        let screen_print_need = (can_print + offset_view) as usize;
        //缓冲区
        let mut out_buf = Vec::with_capacity(screen_print_need as usize);
        let screen_buf_ref = self.screen_buffer.clone();
        let lock_ref = Arc::clone(&self.stdout_lock);
        let view_ref = Arc::clone(&self.screen_view);
        tokio::spawn(async move {
            //统计是否将所有内容全部打印完成
            let mut finished = true;
            //将缓存区内的字符串换行写入缓冲区
            let bufs = screen_buf_ref.read().await;
            for to_print in bufs.iter().rev() {
                let chars = to_print.chars().collect::<Vec<char>>();
                for chuck in chars.chunks(tem_w as usize).rev() {
                    //当打印满时时停止打印入缓冲区
                    if screen_print_need == out_buf.len() {
                        finished = false;
                        break;
                    }
                    out_buf.push(chuck.iter().collect::<String>());
                }
            }
            drop(bufs);
            //输出缓冲区的最后 tem_h -CANT_PRINT_RANGE_HEIGHT 行
            let total_cached_buf_size = out_buf.len();
            // 偏移量计算：当打印的行数小于终端可打印区域高度时为0，否则为相减
            let delta = if total_cached_buf_size as u16 >= can_print {
                total_cached_buf_size as u16 - can_print
            } else {
                0
            };
            let mut stdout = io::stdout();
            let _permit = lock_ref
                .acquire()
                .await
                .expect("Couldn't acquire screen buffer");
            execute!(stdout, cursor::SavePosition);
            for (local, chuck) in out_buf.into_iter().skip(delta as usize).rev().enumerate() {
                execute!(stdout, cursor::MoveTo(0, local as u16));
                execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine));
                execute!(stdout, style::Print(chuck));
            }
            execute!(stdout, cursor::RestorePosition);
            drop(stdout);
            drop(_permit);
            //根据打印结果更新偏移量
            if finished {
                //当内容全部打印完成时，偏移量为打印区域超过可打印区域的范围
                *view_ref.write().await = delta;
            }
        });

        Ok(())
    }
}
