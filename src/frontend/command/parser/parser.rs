use std::net::SocketAddr;
use std::str::SplitWhitespace;
use strum_macros::Display;
use crate::entity::alias::util::InputArgs;
use crate::frontend::command::parser::parser_hd::ParserHandler;

pub enum SystemCall {
    ConnTcpSocket(SocketAddr),
    DisconnectTcpSocket(SocketAddr),
    UnsafeMsgbox(SocketAddr, String),
    ConnStatus,
    Exception(String),
    Unknown,
}
impl SystemCall {
    fn by_name(name: &str, cmd_words: InputArgs) -> Self {
        match name {
            "conn" => ParserHandler::hd_conn_cmd(cmd_words),
            "disconn" => ParserHandler::hd_dis_conn_cmd(cmd_words),
            "connsta" => SystemCall::ConnStatus,
            "msg!" => ParserHandler::hd_unsafe_msgbox(cmd_words),
            _ => SystemCall::Unknown,
        }
    }
}
pub struct CommandParser;
impl CommandParser {
    pub fn parse(command: &str) -> SystemCall {
        let mut cmd_words = command.split_whitespace();
        let cmd_name = match cmd_words.next() {
            None => return SystemCall::Exception("no name given in command".to_string()),
            Some(s) => s,
        };
        SystemCall::by_name(cmd_name, cmd_words)
    }
}
