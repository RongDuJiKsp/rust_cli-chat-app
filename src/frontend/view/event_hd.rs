use crate::main_application::ApplicationLifetime;
use crate::util::char::is_char_printable;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub async fn hd_terminal_event(
    application: &mut ApplicationLifetime,
    screen_event: Event,
) -> anyhow::Result<()> {
    let ctx = &application.printer;
    let app = &application.event_loop;
    //处理按键event
    if let Event::Key(key) = screen_event {
        //处理ctrl+c
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            println!("Ctrl+C pressed, exiting...");
            app.close().await;
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
            KeyCode::Left => {
                ctx.user_view_offset_changed(1).await?;
            }
            KeyCode::Right => {
                ctx.user_view_offset_changed(-1).await?;
            }
            KeyCode::Up => {
                ctx.user_cmd_history(1).await?;
            }
            KeyCode::Down => {
                ctx.user_cmd_history(-1).await?;
            }
            _ => {}
        }
    }
    //TODO:处理其他event
    Ok(())
}
