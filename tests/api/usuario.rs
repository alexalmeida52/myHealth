use crate::helpers::create_app;
use std::collections::HashMap;
use myhealth::controllers::{UsuarioDB, UsuarioId};

#[actix_rt::test]
async fn create_user_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "Josefa Barbosa");
    map.insert("email", "josefabarbosa@gmail.com");
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

    assert_eq!(data.nome, "Josefa Barbosa");
    assert_eq!(data.tipo, "paciente");
    assert_eq!(data.email, "josefabarbosa@gmail.com");
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