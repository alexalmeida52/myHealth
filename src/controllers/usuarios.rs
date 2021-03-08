use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: String,
    pub email: String,
    pub nome: String,
    pub senha: String,
    pub tipo: String,
    pub criado_em: String,
}

pub async fn usuarios() -> impl Responder {
    
    HttpResponse::Ok().json(vec![
        Usuario {
            id: "123".to_string(),
            nome: "Alex".to_string(),
            senha: "ldkasfj".to_string(),
            tipo: "paciente".to_string(),
            criado_em: "2020-10-10".to_string(),
            email: "alex@gmail.com".to_string(),
        },
        Usuario {
            id: "123".to_string(),
            nome: "Fernando".to_string(),
            senha: "fascaad".to_string(),
            tipo: "profissional".to_string(),
            criado_em: "2020-01-10".to_string(),
            email: "fernando@gmail.com".to_string(),
        },
    ])
}
