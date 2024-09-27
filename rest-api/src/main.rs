use actix_web::{App, HttpServer};
use log::info;
use rest_api::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server");

    HttpServer::new(|| {
        App::new().configure(config_app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
