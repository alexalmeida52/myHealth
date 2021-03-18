use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use web::{Data, Json};

#[derive(Deserialize)]
pub struct Usuario {
    nome: String,
    email: String,
    senha: String,
    tipo: String,
}

#[derive(Deserialize)]
pub struct UsuarioUpdate {
    nome: String,
    email: String,
    tipo: String
}

#[derive(Serialize, Deserialize)]
pub struct UsuarioDB {
    pub nome: String,
    pub email: String,
    pub tipo: String,
    pub id: String,
    pub criado_em: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsuarioId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

/**
 * ROTA DE CRIAR UM USUÁRIO
 * [POST] /usuarios
*/
pub async fn usuario(
    usuario: Json<Usuario>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO usuario (id, nome, email, senha, tipo, criado_em)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        usuario.nome,
        usuario.email,
        usuario.senha,
        usuario.tipo,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        let erro = Error {
            message: e.to_string(),
        };

        HttpResponse::Conflict().json(&erro)
    })?;

    let response = UsuarioId { id: id.to_string() };
    Ok(HttpResponse::Ok().json(&response))
}

/**
 * ROTA DE ATUALIZAR UM USUÁRIO
 * [PUT] /usuarios/{id}
*/
pub async fn put_user(
    usuario: Json<UsuarioUpdate>,
    req: HttpRequest,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let id = get_param(req, "id");
    
    sqlx::query!(
        r#"
        UPDATE usuario
        SET nome = $1, email = $2, tipo = $3
        WHERE id = $4
        "#,
        usuario.nome,
        usuario.email,
        usuario.tipo,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        let erro = Error {
            message: e.to_string(),
        };

        HttpResponse::BadRequest().json(&erro)
    })?;

    Ok(HttpResponse::Ok().json("Atualizado com sucesso!"))
}

/**
 * ROTA DE LISTAR TODOS OS USUÁRIOS
 * [DELETE] /usuarios/{id}
*/
pub async fn delete_user(
    req: HttpRequest,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let id = get_param(req, "id");

    sqlx::query!(
        r#"
        DELETE FROM usuario
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json("Removido com sucesso!"))
}

/**
 * ROTA DE LISTAR TODOS OS USUÁRIOS
 * [GET] /usuarios
*/
pub async fn index_user(pool: Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let rows = sqlx::query!(
        r#"
        SELECT id, nome, email, tipo, criado_em
        FROM usuario
        ORDER BY id
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);

        let erro = Error {
            message: e.to_string(),
        };

        HttpResponse::InternalServerError().json(&erro)
    })?;

    let mut usuarios: Vec<UsuarioDB> = Vec::new();
    for row in rows {
        let user = UsuarioDB {
            id: row.id.to_string(),
            nome: row.nome,
            email: row.email,
            tipo: row.tipo,
            criado_em: row.criado_em.to_string(),
        };
        usuarios.push(user);
    }
    
    Ok(HttpResponse::Ok().json(usuarios))
}

/**
 * ROTA DE LISTAR UM USUÁRIO
 * [GET] /usuarios/{id}
*/
pub async fn show_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let id: Uuid = get_param(req, "id");
    
    let row = sqlx::query!("SELECT * FROM usuario WHERE id = $1", id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Falhou to execute query: {}", e);
            let erro = Error {
                message: e.to_string(),
            };
    
            HttpResponse::NotFound().json(&erro)
        })?;

    let response = UsuarioDB {
        nome: row.nome,
        email: row.email,
        tipo: row.tipo,
        criado_em: row.criado_em.to_rfc3339(),
        id: row.id.to_string()
    };

    Ok(HttpResponse::Ok().json(&response))
}

fn get_param(req: HttpRequest, parameter: &str) -> Uuid {
    req.match_info().get(parameter).unwrap().parse().unwrap()
}
