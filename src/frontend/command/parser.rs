use crate::frontend::command::parser_hd::ParserHandler;
use std::net::SocketAddr;
use strum_macros::Display;
#[derive(Display)]
pub enum SystemCall {
    #[strum(to_string = "conn")]
    ConnTcpSocket(SocketAddr),
    #[strum(to_string = "disconn")]
    DisconnectTcpSocket(SocketAddr),
    #[strum(to_string = "msg!")]
    UnsafeMsgbox(SocketAddr, String),
    Exception(String),
    Unknown,
}

pub struct CommandParser {}
impl CommandParser {
    pub fn parse(command: &str) -> SystemCall {
        let mut cmd_words = command.split_whitespace();
        let cmd_name = match cmd_words.next() {
            None => return SystemCall::Exception("no name given in command".to_string()),
            Some(s) => s,
        };
        //hand cmd by name
        match cmd_name {
            "conn" => ParserHandler::hd_conn_cmd(cmd_words),
            "disconn" => ParserHandler::hd_dis_conn_cmd(cmd_words),
            "msg!" => ParserHandler::hd_unsafe_msgbox(cmd_words),
            _ => SystemCall::Unknown,
        }
    }
}
