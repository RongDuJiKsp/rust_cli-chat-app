use crate::entity::alias::util::InputArgs;
use crate::frontend::command::parser::parser::SystemCall;
use crate::frontend::command::parser::tool::{read_addr, read_str};
use std::net::SocketAddr;
use std::str::FromStr;

pub struct ParserHandler;
impl ParserHandler {
    pub fn hd_conn_cmd(mut args: InputArgs) -> SystemCall {
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
    pub fn hd_dis_conn_cmd(mut args: InputArgs) -> SystemCall {
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
    pub fn hd_unsafe_msgbox(mut args: InputArgs) -> SystemCall {
        let addr = match read_addr(&mut args) {
            Ok(d) => d,
            Err(e) => return SystemCall::Exception(e.to_string()),
        };
        let msg = read_str(&mut args);
        if msg.is_empty() {
            return SystemCall::Exception("MsgBox Is Null".to_string());
        }
        SystemCall::UnsafeMsgbox(addr, msg)
    }
    pub fn hd_chat_with(mut args: InputArgs) -> SystemCall {
        let addr = match read_addr(&mut args) {
            Ok(d) => d,
            Err(e) => return SystemCall::Exception(e.to_string()),
        };
        SystemCall::ChatWith(addr)
    }
    pub fn hd_chat_send_msg(mut args: InputArgs) -> SystemCall {
        let msg = read_str(&mut args);
        if msg.is_empty() {
            return SystemCall::Exception("Chat Message Is Null".to_string());
        }
        SystemCall::ChatMsg(msg)
    }
}
