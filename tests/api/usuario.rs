use crate::helpers::create_app;
use std::collections::HashMap;
use myhealth::controllers::{UsuarioDB};
use uuid::Uuid;

#[actix_rt::test]
async fn criar_usuario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "Alex Alves");
    map.insert("email", "alexalves@gmail.com");
    map.insert("senha", "12345");
    map.insert("tipo", "paciente");
    
    let response = client
        .post(&format!("{}/usuarios", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
        
    assert_eq!(200, response.status().as_u16());

    let data = sqlx::query!("SELECT email, nome, tipo FROM usuario",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.nome, "José Alex");
    assert_eq!(data.tipo, "paciente");
    assert_eq!(data.email, "jhosealex@gmail.com");
}

#[actix_rt::test]
async fn atualizar_usuario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    
    let id = "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d";
    let mut user_update = HashMap::new();
    user_update.insert("nome", "Alex Alves");
    user_update.insert("email", "jhosealex@gmail.com");
    user_update.insert("tipo", "profissional");

    let response = client
        .put(&format!("{}/usuarios/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .json(&user_update)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let user_uuid: Uuid = Uuid::parse_str(&id).unwrap();

    let data = sqlx::query!("SELECT email, nome, tipo FROM usuario WHERE id = $1", user_uuid)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.nome, "Alex Alves");
    assert_eq!(data.tipo, "profissional");
    assert_eq!(data.email, "jhosealex@gmail.com");
}

#[actix_rt::test]
async fn remover_usuario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let id = "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d";

    let response_delete_user = client
        .delete(&format!("{}/usuarios/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    let response_show_user = client
        .get(&format!("{}/usuarios/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response_delete_user.status().as_u16());
    assert_eq!(404, response_show_user.status().as_u16());
}

#[actix_rt::test]
async fn listar_usuario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    
    let id = "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d";   
    
    let response = client
        .get(&format!("{}/usuarios/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    let usuario_response_2: UsuarioDB = response.json().await.unwrap();
    
    assert_eq!(usuario_response_2.email, "jhosealex@gmail.com");
    assert_eq!(usuario_response_2.nome, "José Alex");
    assert_eq!(usuario_response_2.tipo, "paciente");

}