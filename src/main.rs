use actix_web::{App, HttpServer};
use cloud_api::genshin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(genshin::scope()))
        .bind(("127.0.0.1", 7878))?
        .run();

    println!("Cloud API is running on port 7878");

    server.await
}
