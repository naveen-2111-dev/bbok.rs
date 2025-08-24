use actix_web::{post, HttpResponse, Responder};

#[post("/signup")]
pub async fn signup() -> impl Responder {
    HttpResponse::Ok().body("User signed up successfully!")
}
