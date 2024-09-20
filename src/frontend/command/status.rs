use std::fmt::Display;

pub struct CommandStatus {
    pub typed_command: u32,
    pub typed_alpha: u32,
    pub last_command: String,
}
impl CommandStatus {
    pub fn new() -> CommandStatus {
        CommandStatus {
            typed_command: 0,
            typed_alpha: 0,
            last_command: String::new(),
        }
    }
}
impl Display for CommandStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "累计输入字符：{}  累计输入命令：{} 最后一次执行的命令：{}",
            self.typed_alpha, self.typed_command, &self.last_command
        )
    }
}
