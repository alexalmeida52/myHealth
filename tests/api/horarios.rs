use crate::helpers::create_app;
use myhealth::controllers::{Horario, HorarioDB};

// // Teste de criar horário
#[actix_rt::test]
async fn criar_horario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let horario: Horario = Horario {
        data_inicio: String::from("2021-04-14T06:00:00Z"),
        data_fim: String::from("2021-04-14T12:00:00Z"),
        dia_da_semana: Some(3),
        duracao_consulta: 60,
        usuario_id: String::from("3c639f2c-ab02-4c54-b751-0bfd7ff5c26d")
    };

    let response = client
        .post(&format!("{}/horarios", &app.address))
        .header("Content-Type", "application/json")
        .json(&horario)
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16())
}

// // Teste de listar horários
#[actix_rt::test]
async fn listar_horarios_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/horarios?usuario_id={}", &app.address, "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16());

    let horarios_response: Vec<HorarioDB> = response.json().await.unwrap();

    assert_eq!(horarios_response.len(), 1);
    
}

// // Teste de remover horário
#[actix_rt::test]
async fn remover_horario_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let id = "3c639f2c-ab02-4c54-b751-0bfd7ff5c26e";

    let response_delete_horario = client
        .delete(&format!("{}/horarios/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response_delete_horario.status().as_u16());

    let response_show_horarios = client
        .get(&format!("{}/horarios?usuario_id={}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response_show_horarios.status().as_u16());

    let horarios_response: Vec<HorarioDB> = response_show_horarios.json().await.unwrap();

    assert_eq!(horarios_response.len(), 0);
}