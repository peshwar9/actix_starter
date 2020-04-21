use actix_web::{web, Error, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

#[derive(Deserialize)]
struct Info {
    userid: u32,
    friend: String,
}

impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        //Create response and set content type
        ready(Ok(HttpResponse::Ok()
     //   .content_type("applicatioj/json")
        .body(body)
    ))
    }
}

async fn index() -> impl Responder {
   // HttpResponse::Ok().body("Hello")
   MyObj {name: "Hello peshwar9"}
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello Again")
}

async fn req_extract_example(info: web::Path<Info>) -> Result<String, Error> {
    Ok(format!("Welcome {} , userid: {}!", info.friend, info.userid))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new().
        service(
        web::scope("/dogood")
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/users/{userid}/{friend}", web::get().to(req_extract_example))
    )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
