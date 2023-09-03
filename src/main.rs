pub mod api;
mod app;
pub mod manifest;
pub mod settings;

use poem::{listener::TcpListener, Server};

use crate::app::build_app;
use crate::settings::get_settings;

#[tokio::main]
async fn main() {
    let settings = get_settings();
    let app = build_app();
    Server::new(TcpListener::bind(settings.host.clone()))
        .run(app)
        .await
        .unwrap();
}
