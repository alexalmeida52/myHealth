use crate::helpers::create_app;
use myhealth::controllers::{ReqBodyPost, HorariosLivresDB};

// Teste de criar agendamento
#[actix_rt::test]
async fn criar_agendamento_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let agendamento: ReqBodyPost = ReqBodyPost {
        data_inicio: String::from("2021-04-11T07:00:00Z"),
        horario_id: String::from("3c639f2c-ab02-4c54-b751-0bfd7ff5c26d"),
        profissional_id: String::from("3c639f2c-ab02-4c54-b751-0bfd7ff5c26d"),
        paciente_id: String::from("3c639f2c-ab02-4c54-b751-0bfd7ff5c26e")
    };

    let response = client
        .post(&format!("{}/agendamentos", &app.address))
        .header("Content-Type", "application/json")
        .json(&agendamento)
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16())
}

// Teste de listar horários
#[actix_rt::test]
async fn listar_disponibilidade_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/agendamentos-livres?profissional_id={}", &app.address, "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16());

    let agendamentos_response: Vec<HorariosLivresDB> = response.json().await.unwrap();

    assert_eq!(agendamentos_response.len(), 5);
    
}

// Teste de remover agendamento
#[actix_rt::test]
async fn remover_agendamento_retorna_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    let id = "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d";

    let response_delete_agendamento = client
        .delete(&format!("{}/agendamentos/{}", &app.address, id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response_delete_agendamento.status().as_u16());

    let response_listar_agendamentos_livres = client
    .get(&format!("{}/agendamentos-livres?profissional_id={}", &app.address, "3c639f2c-ab02-4c54-b751-0bfd7ff5c26d"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(200, response_listar_agendamentos_livres.status().as_u16());

    let agendamentos_livres_response: Vec<HorariosLivresDB> = response_listar_agendamentos_livres.json().await.unwrap();

    assert_eq!(agendamentos_livres_response.len(), 6);
}