use crate::frontend::command::parser::parser::{CommandParser, SystemCall};
use crate::frontend::command::runner::call_hd::CmdCallHandler;
use crate::main_application::ApplicationLifetime;
use crate::util::log_fmt::LogFormatter;

pub struct CommendPlainer {
    app: ApplicationLifetime,
}
impl CommendPlainer {
    pub fn load_app(app: ApplicationLifetime) -> CommendPlainer {
        CommendPlainer { app }
    }
    pub async fn exec_command(&self, command: &str) -> anyhow::Result<CommendExecResult> {
        let mut res = CommendExecResult::new();
        if command.is_empty() {
            res.output
                .append(&mut LogFormatter::error("Command cannot be empty"));
            return Ok(res);
        }
        let sys_call = CommandParser::parse(command);
        match sys_call {
            SystemCall::Unknown => {
                res.output
                    .append(&mut LogFormatter::error("Unknown command"));
            }
            SystemCall::Exception(e) => {
                res.output.append(&mut LogFormatter::error(&format!(
                    "Error on exec command:{}",
                    e
                )));
            }
            SystemCall::ConnTcpSocket(addr) => {
                let mut log = match CmdCallHandler::call_conn(&self.app, addr).await {
                    Ok(_) => LogFormatter::info(&format!("Connection successful to {}", addr)),
                    Err(e) => LogFormatter::error(&format!("Error on Connect Tcp: {}", e)),
                };
                res.output.append(&mut log);
            }
            SystemCall::DisconnectTcpSocket(addr) => {
                let mut log = match CmdCallHandler::call_dis_conn(&self.app, addr).await {
                    Ok(_) => LogFormatter::info("已主动断开连接，将在结束使用后结束连接"),
                    Err(e) => LogFormatter::error(&format!("Error on Disconnect Tcp: {}", e)),
                };
                res.output.append(&mut log);
            }
            SystemCall::UnsafeMsgbox(addr, msg) => {
                let mut log = match CmdCallHandler::call_unsafe_msgbox(&self.app, addr, msg).await {
                    Ok(_) => LogFormatter::info("消息发送成功"),
                    Err(e) => LogFormatter::error(&format!("Error on Send msgbox：{}", e)),
                };
                res.output.append(&mut log);
            }
            SystemCall::ConnStatus => match CmdCallHandler::call_conn_status(&self.app).await {
                Ok(out) => {
                    res.output = out;
                    res.need_clear = true;
                }
                Err(e) => {
                    res.output.append(&mut LogFormatter::error(&format!(
                        "Error on Get connect status：{}",
                        e
                    )));
                }
            },
        }
        Ok(res)
    }
}
pub struct CommendExecResult {
    output: Vec<String>,
    need_clear: bool,
}
impl CommendExecResult {
    fn new() -> CommendExecResult {
        CommendExecResult {
            output: Vec::new(),
            need_clear: false,
        }
    }

    pub fn need_clear(&self) -> bool {
        self.need_clear
    }
    pub fn output(self) -> Vec<String> {
        self.output
    }
}
