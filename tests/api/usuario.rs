use crate::helpers::create_app;

#[actix_rt::test]
async fn subscribe_returns_a_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let body = "name=da%20silva&email=joao_da_silva%40gmail.com";

    let response = client
        .post(&format!("{}/usuario", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}