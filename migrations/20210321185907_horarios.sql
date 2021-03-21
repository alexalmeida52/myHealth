-- Add migration script here
CREATE TABLE horarios(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    data_inicio timestamptz NOT NULL,
	data_fim timestamptz NOT NULL,
	dia_da_semana int DEFAULT -1,
	duracao_consulta int NOT NULL,
	usuario_id uuid NOT NULL,
	criado_em timestamptz NOT NULL,
	CONSTRAINT fk_usuario
      FOREIGN KEY(usuario_id) 
	  REFERENCES usuario(id)
);

INSERT INTO horarios (id, data_inicio, data_fim, duracao_consulta, usuario_id, criado_em)
VALUES('3c639f2c-ab02-4c54-b751-0bfd7ff5c26d', '2021-03-04 06:00:00.468617+00', '2021-03-05 12:00:28.468617+00', 30, '3c639f2c-ab02-4c54-b751-0bfd7ff5c26d', '2021-03-21 16:00:00.468617+00');
