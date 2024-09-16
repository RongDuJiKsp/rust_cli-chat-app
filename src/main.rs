use crate::main_application::MainApplication;

mod backend;
mod config;
mod entity;
mod frontend;
mod main_application;
mod util;

#[tokio::main]
async fn main() {
    let mut app = MainApplication::init()
        .await
        .expect("Could not initialize application:");
    app.run().await.expect("Error while running application:");
    app.destroy()
        .await
        .expect("Error while destroying application:");
}
