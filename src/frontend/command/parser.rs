use std::net::SocketAddr;
use std::str::{FromStr, SplitWhitespace};
use strum_macros::Display;

#[derive(Display)]
pub enum SystemCall {
    #[strum(to_string = "conn")]
    ConnTcpSocket(SocketAddr),
    #[strum(to_string = "disconn")]
    DisconnectTcpSocket(SocketAddr),
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
        match cmd_name {
            "conn" => hd_conn_cmd(cmd_words),
            "disconn" => hd_dis_conn_cmd(cmd_words),
            _ => SystemCall::Unknown,
        }
    }
}
fn hd_conn_cmd(mut args: SplitWhitespace) -> SystemCall {
    let addr = match args.next() {
        None => {
            return SystemCall::Exception("no addr given in conn".to_string());
        }
        Some(s) => s,
    };
    match SocketAddr::from_str(addr) {
        Err(e) => SystemCall::Exception(format!("invalid addr: {}", e.to_string())),
        Ok(addr) => SystemCall::ConnTcpSocket(addr),
    }
}
fn hd_dis_conn_cmd(mut args: SplitWhitespace) -> SystemCall {
    let addr = match args.next() {
        None => {
            return SystemCall::Exception("no addr given in conn".to_string());
        }
        Some(s) => s,
    };
    match SocketAddr::from_str(addr) {
        Err(e) => SystemCall::Exception(format!("invalid addr: {}", e.to_string())),
        Ok(addr) => SystemCall::DisconnectTcpSocket(addr),
    }
}
