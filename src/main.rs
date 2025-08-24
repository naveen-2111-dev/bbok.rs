use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use devmet_project::routes::signup::signup;
use devmet_project::middlewares::guard;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(signup)
            .service(
                web::scope("/api")
                    .wrap(middleware::from_fn(guard))
                    .service(echo)
            )
            .service(manual_hello)
    })
    .bind(("127.0.0.1", 8071))?
    .run()
    .await
}