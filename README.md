# myHealth

## Autor
[José Alex Alves Pereira](https://alexalves.herokuapp.com)

## Projeto da disciplina de Desenvolvimento Web II

O myHealth é uma API REST desenvolvida em RUST que permitirá a comunicação entre outras aplicações frontend ou backend. Utilizarei um app mobile desenvolvido em React Native para consumir os endpoints da api, mas pode ficar a vontade para realizar chamadas para essa api de outras maneiras, como uma interface web ou uma aplicação de criar chamadas HTTP como o Postman ou Insomnia.

### Sobre o myHealth
Este é um software para a área da saúde, que permite pacientes marcarem consultas com apenas alguns cliques, deixando no passado o trabalhosa forma de marcação, que é prencialmente ou realizando uma ligação telefonica. Pelo app será possível realizar as seguintes operações:

- [ ] Criar usuário paciente ou profissional
- [ ] Autenticar usuário
- [ ] Listar usuários do tipo profissional
- [ ] Criar grades de horários de um profissional
- [ ] Listar grades de horários de um profissional
- [ ] Editar uma grade de horários de um profissional
- [ ] Remover grades de horários de um profissional
- [ ] Remover uma grade de horários de um profissional
- [ ] Criar agendamentos de um paciente para um profissional
- [ ] Remover agendamentos de um paciente/profissional

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

## Licença
[MIT](https://choosealicense.com/licenses/mit/)
