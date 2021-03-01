use actix_web::{HttpResponse};

pub async fn users() -> HttpResponse {
    HttpResponse::Ok().body("Listar todos os usuários")
} 