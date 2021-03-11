use crate::helpers::create_app;
use std::collections::HashMap;

#[actix_rt::test]
async fn usuario_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("nome", "Josefa Barbosa");
    map.insert("email", "josefabarbosa@gmail.com");
    map.insert("senha", "12345");
    map.insert("tipo", "paciente");
    
    let response = client
        .post(&format!("{}/usuario", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
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