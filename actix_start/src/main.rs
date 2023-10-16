use std::env;
use actix_web::{App, HttpServer};

mod hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(hello::init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
