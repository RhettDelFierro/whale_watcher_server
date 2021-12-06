-- Add migration script here
CREATE TABLE holder_totals
(
    transaction_id SERIAL UNIQUE PRIMARY KEY,
    network_id     INTEGER     NOT NULL,
    address        TEXT        NOT NULL,
    token_name_id  INTEGER     NOT NULL,
    place          INTEGER     NOT NULL,
    amount         DECIMAL     NOT NULL,
    checked_on     timestamptz NOT NULL,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address),
    FOREIGN KEY (token_name_id) REFERENCES token_names (token_name_id)
);

CREATE TYPE scam_types AS ENUM ('honeypot', 'liquidity_pull', 'rug_pull');
CREATE TYPE holder_types AS ENUM ('whale', 'liquidity locker', 'exchange', 'paperhand', 'dumper', 'scammer');

CREATE TABLE scam_token_creators
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    network_of_scammed_token TEXT NOT NULL,
    scammed_contract_address TEXT NOT NULL,
    FOREIGN KEY (network_of_scammed_token, scammed_contract_address) REFERENCES addresses(network_id, address) ,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE scam_tokens
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    scam_creator_network TEXT NOT NULL,
    scam_creator_address TEXT NOT NULL,
    FOREIGN KEY (scam_creator_network, scam_creator_address) REFERENCES addresses(network_id, address),
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE legit_whales
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE paper_hands
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE dumpers
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);