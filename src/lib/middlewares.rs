use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpResponse,
};
use actix_web::cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn guard(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse, Error> {
    let token_opt = req.cookie("auth_token").map(|c: Cookie| c.value().to_string());

    match token_opt {
        Some(token) => {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let validation = Validation::default();

            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            );

            match decoded {
                Ok(_) => next.call(req).await.map(|res| res.map_into_boxed_body()),
                Err(_) => Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body("Invalid or expired token")
                        .map_into_boxed_body(),
                )),
            }
        }
        None => Ok(req.into_response(
            HttpResponse::Unauthorized()
                .body("Missing auth token")
                .map_into_boxed_body(),
        )),
    }
}
