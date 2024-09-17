use crate::util::log_fmt::LogFormatter;

pub struct CommendPlainer {}
impl CommendPlainer {
    pub async fn exec_command(command: &str) -> anyhow::Result<CommendExecResult> {
        let mut res = CommendExecResult::new();
        if command.is_empty() {
            res.output
                .append(&mut LogFormatter::error("Command cannot be empty"));
        } else {
            res.output.append(&mut LogFormatter::info(&format!(
                "You Run The Command: {}",
                command
            )));
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
