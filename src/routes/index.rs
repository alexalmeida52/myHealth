
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use crate::controllers::{user, users};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/user", web::post().to(user))
                .route("/users", web::get().to(users))
    })
    .bind("127.0.0.1:3000")?
    .run();
    Ok(server)
}