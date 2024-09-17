use crate::main_application::MainApplication;

mod backend;
mod config;
mod entity;
mod frontend;
mod main_application;
mod util;

#[tokio::main]
async fn main() {
    let res = run_application().await;
    MainApplication::defer_finally().await;
    match res {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
async fn run_application() -> anyhow::Result<()> {
    let mut app = MainApplication::init()
        .await
        .map_err(|e| anyhow::anyhow!("Could not initialize application: {}", e))?;
    app.run()
        .await
        .map_err(|e| anyhow::anyhow!("Error while running application:{}", e))?;
    app.destroy()
        .await
        .map_err(|e| anyhow::anyhow!("Error while destroying application:{}", e))?;
    Ok(())
}
