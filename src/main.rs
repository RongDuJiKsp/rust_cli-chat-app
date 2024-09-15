use crate::main_application::MainApplication;

mod entity;
mod main_application;
mod util;
#[tokio::main]
async fn main() {
    let app = MainApplication::init().expect("Could not initialize application:");
    app.run().await.expect("Error while running application:");
}
