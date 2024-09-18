use crate::frontend::command::parser::{CommandParser, SystemCall};
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
                res.output.append(&mut LogFormatter::error("Unknown command"));
            }
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
