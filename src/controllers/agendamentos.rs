use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Utc, DateTime, Duration};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use web::{Data, Json};

#[derive(Serialize, Deserialize)]
pub struct ReqBodyPost {
    pub data_inicio: String,
    pub horario_id: String,
    pub paciente_id: String,
    pub profissional_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    message: String
}

#[derive(Serialize, Deserialize)]
pub struct AgendamentoDB {
    pub id: Uuid,
    pub data_inicio: DateTime<Utc>,
    pub horario_id: Uuid,
    pub paciente_id: Uuid,
    pub profissional_id: Uuid,
    pub criado_em: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct QueryParamsAgendamento {
    pub profissional_id: String,
    // pub horario_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct HorariosLivresDB {
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub duracao_consulta: i32,
    pub horario_id: Uuid
}

#[derive(Serialize, Deserialize)]
struct HorarioDB {
    id: Uuid,
    data_inicio: DateTime<Utc>,
    data_fim: DateTime<Utc>,
    usuario_id: Uuid,
    dia_da_semana: Option<i32>,
    duracao_consulta: i32,
    criado_em: DateTime<Utc>
}

/**
 * ROTA DE CRIAR AGENDAMENTO
 * [POST] /agendamentos
*/
pub async fn criar_agendamento(
    agendamento: Json<ReqBodyPost>,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let id = Uuid::new_v4();
    let criado_em = Utc::now();

    let data_inicio = agendamento.data_inicio.parse::<DateTime<Utc>>();
    let horario_id = Uuid::parse_str(&agendamento.horario_id).unwrap();
    let profissional_id = Uuid::parse_str(&agendamento.profissional_id).unwrap();
    let paciente_id = Uuid::parse_str(&agendamento.paciente_id).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO agendamentos (id, data_inicio, horario_id, paciente_id, profissional_id, criado_em)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        data_inicio.unwrap(),
        horario_id,
        paciente_id,
        profissional_id,
        criado_em
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

    Ok(HttpResponse::Ok().json(&{
        id
    }))
}

/**
 * ROTA DE LISTAR A DISPONIBILIDADE DE UM USUÁRIO PROFISSIONAL
 * [GET] /agendamentos/{usuario_id}
*/
pub async fn listar_disponibilidade_do_profissional(
    req_query: web::Query<QueryParamsAgendamento>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    
    let profissional_id: Uuid = Uuid::parse_str(&req_query.profissional_id).unwrap();
    // let horario_id: Uuid = Uuid::parse_str(&req_query.horario_id).unwrap();
    
    let rows_agendamentos = sqlx::query_as!(AgendamentoDB,"SELECT * FROM agendamentos WHERE profissional_id = $1", profissional_id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Falhou to execute query: {}", e);
            let erro = Error {
                message: e.to_string(),
            };
    
            HttpResponse::NotFound().json(&erro)
        })?;
    
    let rows_horarios = sqlx::query_as!(HorarioDB,"SELECT * FROM horarios WHERE usuario_id = $1", profissional_id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Falhou to execute query: {}", e);
            let erro = Error {
                message: e.to_string(),
            };
    
            HttpResponse::NotFound().json(&erro)
        })?;

    let mut horarios_livres: Vec<HorariosLivresDB> = Vec::new();
    for row in rows_horarios {
        let mut data_inicio = row.data_inicio;
        let data_fim = row.data_fim;
        while data_inicio < data_fim {
            let mut esta_livre: bool = true;
            for agendamento in &rows_agendamentos {
                
                if agendamento.data_inicio == data_inicio {
                    esta_livre = false;
                    break;
                }
            }
            if esta_livre {
                let horario_livre = HorariosLivresDB {
                    data_inicio: data_inicio,
                    data_fim: data_inicio + Duration::minutes(row.duracao_consulta.into()),
                    duracao_consulta: row.duracao_consulta,
                    horario_id: row.id
                };
                horarios_livres.push(horario_livre);
            }
            data_inicio = data_inicio + Duration::minutes(row.duracao_consulta.into());
        }
    }
    Ok(HttpResponse::Ok().json(horarios_livres))
}

/**
 * ROTA DE DELETAR UM AGENDAMENTO
 * [DELETE] /agendamentos/{id}
*/
pub async fn remover_agendamento(
    req: HttpRequest,
    pool: Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
    let id = get_param(req, "id");

    sqlx::query!(
        r#"
        DELETE FROM agendamentos
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

fn get_param(req: HttpRequest, parameter: &str) -> Uuid {
    req.match_info().get(parameter).unwrap().parse().unwrap()
}