use actix_web::{get, web, Responder, Scope};

pub fn scope() -> Scope {
    web::scope("/genshin").service(info).service(time)
}

#[get("/info")]
async fn info() -> impl Responder {
    "info"
}

#[get("/time")]
async fn time() -> impl Responder {
    "time"
}
