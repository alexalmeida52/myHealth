
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use crate::controllers::{usuario, users};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/usuario", web::post().to(usuario))
            .route("/users", web::post().to(users))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
} 