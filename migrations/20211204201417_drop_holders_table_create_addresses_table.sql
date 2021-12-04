-- Add migration script here
DROP TABLE holders;

CREATE TABLE networks
(
    network_id SERIAL PRIMARY KEY,
    token_name TEXT NOT NULL UNIQUE
);