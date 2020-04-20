use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("JGD")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("SGD")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new().
        service(
        web::scope("/dogood")
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
