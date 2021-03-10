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
    println!("{}", response.status().as_u16());
    assert_eq!(200, response.status().as_u16());
}