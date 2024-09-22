use std::fmt::{Display, Formatter};
use std::net::SocketAddr;

pub struct MetaSocketAddr {
    addr: SocketAddr,
    meta: String,
}
impl MetaSocketAddr {
    pub fn pkg(addr: SocketAddr, meta: String) -> Self {
        Self { addr, meta }
    }
}
impl Display for MetaSocketAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.addr.to_string(), self.meta)?;
        Ok(())
    }
}