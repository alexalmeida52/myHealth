-- Add migration script here
CREATE TABLE usuario(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    nome TEXT NOT NULL,
    senha TEXT NOT NULL,
    tipo TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    criado_em timestamptz NOT NULL
)