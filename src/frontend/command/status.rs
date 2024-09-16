use std::fmt::Display;

pub struct CommandStatusCtx {
    pub typed_command: u32,
    pub typed_alpha: u32,
}
impl CommandStatusCtx {
    pub fn new() -> CommandStatusCtx {
        CommandStatusCtx { typed_command: 0, typed_alpha: 0 }
    }
}
impl Display for CommandStatusCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "累计输入字符：{}  累计输入命令：{}", self.typed_alpha, self.typed_command)
    }
}