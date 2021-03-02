use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Usuario {
    nome: String,
    email: String,
    senha: String,
    tipo: String
}

pub async fn usuario(
    usuario: web::Form<Usuario>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    println!("Tentando inserir");
    sqlx::query!(
        r#"
        INSERT INTO usuario (id, nome, email, senha, tipo, criado_em)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        usuario.nome,
        usuario.email,
        usuario.senha,
        usuario.tipo,
        Utc::now()
    )
    
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        println!("Falha...");
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    println!("Sucesso...");
    Ok(HttpResponse::Ok().finish())
} 