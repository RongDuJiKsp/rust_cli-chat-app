use crate::main_application::MainApplication;

mod entity;
mod main_application;
mod util;
mod view;
mod config;
mod connect;

#[tokio::main]
async fn main() {
    let app = MainApplication::init().await.expect("Could not initialize application:");
    app.run().await.expect("Error while running application:");
}
