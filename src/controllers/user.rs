use actix_web::{HttpResponse};

pub async fn user() -> HttpResponse {
    HttpResponse::Ok().body("Criar um usuário")
} 