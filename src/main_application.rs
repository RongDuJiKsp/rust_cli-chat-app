use clap::Parser;

#[derive(Parser)]
struct ApplicationArgs {
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,
    #[clap(short = 'b', long = "bind_ip")]
    bind_ip: Option<String>,
    #[clap(short = 'n', long = "nick")]
    nick: Option<String>,
    #[clap(short = "buf", long = "channel_size", default_value = 1024)]
    channel_size: usize,
}

pub struct MainApplication {
    args: ApplicationArgs,
    listener_port: u16,
}
impl MainApplication {
    pub fn init() -> anyhow::Result<MainApplication> {
        let args = ApplicationArgs::parse();
        let listener_port = args.port.unwrap_or_else(|| 0);
        Ok(MainApplication { args, listener_port })
    }
    pub async fn run(&self) -> anyhow::Result<()> {

        Ok(())
    }
}
