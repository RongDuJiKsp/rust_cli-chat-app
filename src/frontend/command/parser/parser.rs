use crate::entity::alias::util::InputArgs;
use crate::frontend::command::parser::parser_hd::ParserHandler;
use std::net::SocketAddr;

pub enum SystemCall {
    ConnTcpSocket(SocketAddr),
    DisconnectTcpSocket(SocketAddr),
    UnsafeMsgbox(SocketAddr, String),
    ConnStatus,
    ChatWith(SocketAddr),
    ChatMsg(String),
    Exception(String),
    Unknown,
}
impl SystemCall {
    fn by_name(name: &str, cmd_words: InputArgs) -> Self {
        match name {
            "conn" => ParserHandler::hd_conn_cmd(cmd_words),
            "disconn" => ParserHandler::hd_dis_conn_cmd(cmd_words),
            "sta!" => SystemCall::ConnStatus,
            "connsta" => SystemCall::ConnStatus,
            "msg!" => ParserHandler::hd_unsafe_msgbox(cmd_words),
            "msgbox" => ParserHandler::hd_unsafe_msgbox(cmd_words),
            "cw!" => ParserHandler::hd_chat_with(cmd_words),
            "chatwith" => ParserHandler::hd_chat_with(cmd_words),
            "chat!" => ParserHandler::hd_chat_send_msg(cmd_words),
            "chatmsg" => ParserHandler::hd_chat_send_msg(cmd_words),
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
