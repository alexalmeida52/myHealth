use crate::helpers::create_app;
use std::collections::HashMap;
use myhealth::controllers::{UsuarioDB, UsuarioId};
use uuid::Uuid;

#[actix_rt::test]
async fn create_user_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "José Alex");
    map.insert("email", "jhosealex@gmail.com");
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
async fn update_user_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "José Alex");
    map.insert("email", "jhosealex@gmail.com");
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

    let create_response: UsuarioId = response.json().await.unwrap();

    let mut user_update = HashMap::new();
    user_update.insert("nome", "Alex Alves");
    user_update.insert("email", "jhosealex@gmail.com");
    user_update.insert("tipo", "profissional");

    let response_2 = client
        .put(&format!("{}/usuarios/{}", &app.address, create_response.id))
        .header("Content-Type", "application/json")
        .json(&user_update)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response_2.status().as_u16());

    let user_uuid: Uuid = Uuid::parse_str(&create_response.id).unwrap();

    let data = sqlx::query!("SELECT email, nome, tipo FROM usuario WHERE id = $1", user_uuid)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(data.nome, "Alex Alves");
    assert_eq!(data.tipo, "profissional");
    assert_eq!(data.email, "jhosealex@gmail.com");
}

#[actix_rt::test]
async fn show_user_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    
    let mut map = HashMap::new();
        map.insert("nome", "José Alex");
        map.insert("email", "jhosealex@gmail.com");
        map.insert("senha", "12345");
        map.insert("tipo", "paciente");

    let response_1 = client
        .post(&format!("{}/usuarios", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    
    let usuario_response_1: UsuarioId = response_1.json().await.unwrap();
    
    let response_2 = client
        .get(&format!("{}/usuarios/{}", &app.address, usuario_response_1.id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    let usuario_response_2: UsuarioDB = response_2.json().await.unwrap();
    
    assert_eq!(usuario_response_2.email, "jhosealex@gmail.com");
    assert_eq!(usuario_response_2.nome, "José Alex");
    assert_eq!(usuario_response_2.tipo, "paciente");

}