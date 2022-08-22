use crate::{python, typedef::genshin::*};
use actix_web::{
    get,
    http::header::ContentType,
    web::{self, Query},
    HttpResponse, Responder, Scope,
};

pub fn scope() -> Scope {
    web::scope("/genshin").service(info).service(time)
}

#[get("/info")]
async fn info(query: Query<QueryWithCookie>) -> impl Responder {
    run_py(INFO_PY, &query.cookie)
}

#[get("/time")]
async fn time(query: Query<QueryWithCookie>) -> impl Responder {
    run_py(TIME_PY, &query.cookie)
}

const INFO_PY: &str = include_str!("../python/genshin_info.py");
const TIME_PY: &str = include_str!("../python/genshin_time.py");

fn run_py(code: &str, cookie: &str) -> impl Responder {
    match python::exec_py(code, vec![cookie]) {
        Ok(output) => {
            if output.status.success() {
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(output.stdout)
            } else {
                HttpResponse::BadRequest().body(output.stderr)
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::{INFO_PY, TIME_PY};
    use crate::python;

    #[test]
    fn test_info() {
        let result =
            python::exec_py(INFO_PY, vec![include_str!("../private/test_cookie.txt")]).unwrap();
        println!("{:?}", result)
    }

    #[test]
    fn test_time() {
        let result =
            python::exec_py(TIME_PY, vec![include_str!("../private/test_cookie.txt")]).unwrap();
        println!("{:?}", result)
    }
}
