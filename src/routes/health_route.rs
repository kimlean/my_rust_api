use actix_web::{get, HttpResponse, Responder};
use crate::services::health_service;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body(health_service::health_check())
}
