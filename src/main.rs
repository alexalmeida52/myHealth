use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn user() -> impl Responder {
    // TODO código para criar um usuário no banco de dados
    HttpResponse::Ok().body("Create user!")
}

async fn users() -> impl Responder {
    // TODO código para listar os usuários no banco de dados
    HttpResponse::Ok().body("List users!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user", web::post().to(user))
            .route("/users", web::get().to(users))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}