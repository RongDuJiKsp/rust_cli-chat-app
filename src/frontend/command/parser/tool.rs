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
pub fn read_str(args: &mut InputArgs, typed: &str) -> anyhow::Result<String> {
    match args.next() {
        None => Err(anyhow::anyhow!("No {} Param Given", typed)),
        Some(s) => Ok(s.to_string()),
    }
}
