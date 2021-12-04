-- Add migration script here
CREATE TABLE addresses
(
    address_id SERIAL NOT NULL,
    network_id integer REFERENCES networks (network_id),
    address    TEXT   NOT NULL UNIQUE,
    PRIMARY KEY (address_id, network_id)
);

CREATE TABLE token_names
(
    token_name_id SERIAL PRIMARY KEY,
    token_name    TEXT NOT NULL UNIQUE
);