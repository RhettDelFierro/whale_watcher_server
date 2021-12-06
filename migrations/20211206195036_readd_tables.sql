-- Add migration script here
CREATE TABLE networks
(
    network_id   SERIAL UNIQUE PRIMARY KEY,
    network_name TEXT NOT NULL UNIQUE
);

CREATE TABLE token_names
(
    token_name_id SERIAL UNIQUE PRIMARY KEY,
    token_name    TEXT NOT NULL UNIQUE
);

CREATE TABLE addresses
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    PRIMARY KEY (network_id, address)
);

CREATE TABLE address_token_names
(
    network_id    integer not null,
    address       TEXT    NOT NULL,
    token_name_id integer not null,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address),
    FOREIGN KEY (token_name_id) REFERENCES token_names (token_name_id)
);