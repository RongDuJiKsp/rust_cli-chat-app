use crate::frontend::command::parser::SystemCall;
use std::net::SocketAddr;
use std::str::{FromStr, SplitWhitespace};
pub struct ParserHandler;
impl ParserHandler {
    pub fn hd_conn_cmd(mut args: SplitWhitespace) -> SystemCall {
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
    pub fn hd_dis_conn_cmd(mut args: SplitWhitespace) -> SystemCall {
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
    pub fn hd_unsafe_msgbox(mut args: SplitWhitespace) -> SystemCall {
        let addr = match args
            .next()
            .ok_or(anyhow::anyhow!("no addr given"))
            .and_then(SocketAddr::from_str)
        {
            Ok(a) => a,
            Err(e) => return SystemCall::Exception(format!("error on addr: {}", e.to_string())),
        };
        let msg = match args.next() {
            None => return SystemCall::Exception("no msg given".to_string()),
            Some(s) => s.to_string(),
        };
        SystemCall::UnsafeMsgbox(addr, msg)
    }
}
