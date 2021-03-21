use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use web::{Data, Json};

#[derive(Deserialize)]
pub struct Horario {
    data_inicio: String,
    data_fim: String,
    usuario_id: String,
    dia_da_semana: Option<i32>,
    duracao_consulta: i32,
}

#[derive(Serialize, Deserialize)]
pub struct HorarioDB {
    id: Uuid,
    data_inicio: DateTime<Utc>,
    data_fim: DateTime<Utc>,
    usuario_id: Uuid,
    dia_da_semana: Option<i32>,
    duracao_consulta: i32,
    criado_em: DateTime<Utc>
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