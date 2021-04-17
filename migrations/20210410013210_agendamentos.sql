-- Add migration script here
CREATE TABLE agendamentos(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    data_inicio timestamptz NOT NULL,
    horario_id uuid NOT NULL,
	paciente_id uuid NOT NULL,
	profissional_id uuid NOT NULL,
	criado_em timestamptz NOT NULL,
	CONSTRAINT fk_usuario_paciente
      FOREIGN KEY(paciente_id) 
	  REFERENCES usuario(id),
    CONSTRAINT fk_usuario_profissional
      FOREIGN KEY(profissional_id) 
	  REFERENCES usuario(id),
    CONSTRAINT fk_horarios
      FOREIGN KEY(horario_id) 
	  REFERENCES horarios(id)
);


INSERT INTO agendamentos (id, data_inicio, horario_id, paciente_id, profissional_id, criado_em)
VALUES(
	'3c639f2c-ab02-4c54-b751-0bfd7ff5c26d', 
	'2021-03-04 06:00:00.468617+00',
	'3c639f2c-ab02-4c54-b751-0bfd7ff5c26d',
	'3c639f2c-ab02-4c54-b751-0bfd7ff5c26d',
	'3c639f2c-ab02-4c54-b751-0bfd7ff5c26d',
	'2021-03-03 06:00:00.468617+00'
);