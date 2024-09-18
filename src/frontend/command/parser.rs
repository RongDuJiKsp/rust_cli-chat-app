pub enum SystemCall {
    Unknown
}
pub struct CommandParser {}
impl CommandParser {
    pub fn parse(command: &str) -> SystemCall {
        SystemCall::Unknown
    }
}