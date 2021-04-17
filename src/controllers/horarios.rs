use actix_web::{web, HttpResponse, HttpRequest};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use web::{Data, Json};

#[derive(Serialize, Deserialize)]
pub struct Horario {
    pub data_inicio: String,
    pub data_fim: String,
    pub usuario_id: String,
    pub dia_da_semana: Option<i32>,
    pub duracao_consulta: i32,
}

#[derive(Serialize, Deserialize)]
pub struct HorarioDB {
    pub id: Uuid,
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub usuario_id: Uuid,
    pub dia_da_semana: Option<i32>,
    pub duracao_consulta: i32,
    pub criado_em: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct HorarioId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct HorarioUsuarioId {
    pub usuario_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

/**
 * ROTA DE CRIAR UM HORÁRIO
 * [POST] /horarios
*/
pub async fn criar_horario(
    horario: Json<Horario>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let id = Uuid::new_v4();
    let data_inicio = horario.data_inicio.parse::<DateTime<Utc>>();
    let data_fim = horario.data_fim.parse::<DateTime<Utc>>();
    sqlx::query!(
        r#"
        INSERT INTO horarios (id, data_inicio, data_fim, dia_da_semana, duracao_consulta, usuario_id, criado_em)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        data_inicio.unwrap(),
        data_fim.unwrap(),
        horario.dia_da_semana,
        horario.duracao_consulta,
        Uuid::parse_str(&horario.usuario_id).unwrap(),
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

    let response = HorarioId { id: id.to_string() };
    Ok(HttpResponse::Ok().json(&response))
}

/**
 * ROTA DE LISTAR TODOS OS HORÁRIOS DE UM USUÁRIO PROFISSIONAL
 * [GET] /horarios/{usuario_id}
*/
pub async fn listar_horarios_do_profissional(
    req_query: web::Query<HorarioUsuarioId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    
    let usuario_id: Uuid = Uuid::parse_str(&req_query.usuario_id).unwrap();
    
    let rows = sqlx::query_as!(HorarioDB,"SELECT * FROM horarios WHERE usuario_id = $1", usuario_id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Falhou to execute query: {}", e);
            let erro = Error {
                message: e.to_string(),
            };
    
            HttpResponse::NotFound().json(&erro)
        })?;
   
    Ok(HttpResponse::Ok().json(rows))
}


/**
 * ROTA DE DELETAR UM HORÁRIO
 * [DELETE] /horarios/{id}
*/
pub async fn remover_horario(
    req: HttpRequest,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let id = get_param(req, "id");

    sqlx::query!(
        r#"
        DELETE FROM horarios
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::BadRequest().json(e.to_string())
    })?;

    Ok(HttpResponse::Ok().json("Horário removido com sucesso!"))
}

/**
 * ROTA DE DELETAR TODOS HORÁRIOS
 * [DELETE] /horarios/{id}
*/
pub async fn remover_todos_horarios_do_profissional(
    req: HttpRequest,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let profissional_id = get_param(req, "profissional_id");

    sqlx::query!(
        r#"
        DELETE FROM horarios
        WHERE usuario_id = $1
        "#,
        profissional_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::BadRequest().json(e.to_string())
    })?;

    Ok(HttpResponse::Ok().json("Todos os horários removidos com sucesso!"))
}

fn get_param(req: HttpRequest, parameter: &str) -> Uuid {
    req.match_info().get(parameter).unwrap().parse().unwrap()
}