# myHealth

## Autor
[José Alex Alves Pereira](https://alexalves.herokuapp.com)

<details open="open">
  <summary>Sumário</summary>
  <ol>
    <li>
      <a href="#apresentação">Apresentação</a>
    </li>
    <li>
      <a href="#sobre-o-myHealth">Sobre</a>
    </li>
    <li>
      <a href="#estórias-de-usuário">Estórias de usuário</a>
    </li>
    <li>
      <a href="#endpoints">Endpoints</a>
    </li>
    <li>
      <a href="#licença">Licença</a>
    </li>
  </ol>
</details>

## Apresentação
### Projeto da disciplina de Desenvolvimento Web II

O myHealth é uma API REST desenvolvida em RUST que permitirá a comunicação entre outras aplicações frontend ou backend. Utilizarei um app mobile desenvolvido em React Native para consumir os endpoints da api, mas pode ficar a vontade para realizar chamadas para essa api de outras maneiras, como uma interface web ou uma aplicação de criar chamadas HTTP como o Postman ou Insomnia.

### Sobre o myHealth
Este é um software para a área da saúde, que permite pacientes marcarem consultas com apenas alguns cliques, deixando no passado o trabalhosa forma de marcação, que é prencialmente ou realizando uma ligação telefonica.

### Estórias de usuário
- Sendo um paciente, eu desejo:
  - Visualizar uma lista de profissionais.
  - Visualizar detalhes de um profissional.
  - Marcar consultas para um profissional.
  - Cancelar uma consulta.
- Sendo um profissional, eu desejo:
  - Receber marcações de consultas.
  - Cancelar marcações de consultas.
  - Criar marcações de consultas para um paciente.
  - Criar grades de horários.
  - Editar grades de horários.
  - Remover grades de horários.
  - Remover uma grade de horários.

### EndPoints
#### ``Usuário``
  - [ ] [ POST ] ``/auth`` -> Autenticar usuário
  - [x] [ POST ] ``/usuarios`` -> Criar usuário paciente ou profissional
  - [x] [ PUT ] ``/usuarios`` -> Atualizar um usuário paciente ou profissional
  - [x] [ DELETE ] ``/usuarios`` -> Remover um usuário paciente ou profissional
  - [x] [ GET ] ``/usuarios`` -> Listar usuários
  - [x] [ GET ] ``/usuarios`` -> Listar um usuário
  
#### ``Horários``
  - [ ] [ POST ] ``/horarios/grade`` -> Criar grade de horários de um profissional
  - [ ] [ GET] ``/horarios/grades`` -> Listar grades de horários de um profissional
  - [ ] [ PUT ] ``/horarios/grade/:id`` -> Editar uma grade de horários de um profissional
  - [ ] [ DELETE ] ``/horarios/grades`` -> Remover grades de horários de um profissional
  - [ ] [ DELETE ] ``/horarios/grade/:id`` -> Remover uma grade de horários de um profissional

#### ``Agendamentos``
  - [ ] [ POST ] ``/agendamento`` -> Criar agendamentos de um paciente para um profissional
  - [ ] [ DELETE ] ``/agendamentos/:id`` -> Remover agendamentos de um paciente/profissional

### Licença
[MIT](https://choosealicense.com/licenses/mit/)
