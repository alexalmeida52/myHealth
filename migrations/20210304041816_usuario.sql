-- Add migration script here
CREATE TABLE usuario(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    nome TEXT NOT NULL,
    senha TEXT NOT NULL,
    tipo TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    criado_em timestamptz NOT NULL
);

INSERT INTO usuario
VALUES('3c639f2c-ab02-4c54-b751-0bfd7ff5c26d', 'José Alex', '12345', 'profissional', 'jhosealex@gmail.com', '2021-03-04 05:15:28.468617+00');

INSERT INTO usuario
VALUES('3c639f2c-ab02-4c54-b751-0bfd7ff5c26e', 'João Das Couves', '12345', 'paciente', 'joao@gmail.com', '2021-03-04 05:15:28.468617+00');