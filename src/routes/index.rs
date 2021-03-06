
use crate::configuration::{DatabaseSettings, Settings};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use actix_web::web::Data;
use crate::controllers::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.with_db())
        .await
}

fn run(
    listener: TcpListener, 
    db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            // ROTAS DE USUÁRIOS
            .route("/usuarios", web::post().to(criar_usuario))
            .route("/usuarios", web::get().to(listar_usuarios))
            .route("/usuarios/{id}", web::get().to(listar_usuario))
            .route("/usuarios/{id}", web::put().to(atualizar_usuario))
            .route("/usuarios/{id}", web::delete().to(remover_usuario))

            // ROTAS DE HORÁRIOS
            .route("/horarios", web::post().to(criar_horario))
            .route("/horarios", web::get().to(listar_horarios_do_profissional))
            .route("/horarios/{id}", web::delete().to(remover_horario))
            .route("/horarios-profissional/{profissional_id}", web::delete().to(remover_todos_horarios_do_profissional))

            // ROTAS DE AGENDAMENTOS
            .route("/agendamentos", web::post().to(criar_agendamento))
            .route("/agendamentos-livres", web::get().to(listar_disponibilidade_do_profissional))
            .route("/agendamentos/{id}", web::delete().to(remover_agendamento))

            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
} 