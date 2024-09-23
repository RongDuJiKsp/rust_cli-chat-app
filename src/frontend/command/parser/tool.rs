use crate::entity::alias::util::InputArgs;
use std::net::SocketAddr;
use std::str::FromStr;

pub fn read_addr(args: &mut InputArgs) -> anyhow::Result<SocketAddr> {
    args.next()
        .ok_or(anyhow::anyhow!("no addr given"))
        .and_then(|e| match SocketAddr::from_str(e) {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        })
}
pub fn read_str(args: &mut InputArgs) -> String {
    args.map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
