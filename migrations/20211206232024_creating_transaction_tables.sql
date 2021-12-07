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
    network_of_scammed_token INTEGER NOT NULL,
    scammed_contract_address TEXT NOT NULL,
    FOREIGN KEY (network_of_scammed_token, scammed_contract_address) REFERENCES addresses(network_id, address) ,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE scam_tokens
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    scam_creator_network INTEGER NOT NULL,
    scam_creator_address TEXT NOT NULL,
    scam_type scam_types,
    FOREIGN KEY (scam_creator_network, scam_creator_address) REFERENCES addresses(network_id, address),
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE legit_token_creators
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    network_of_major_token INTEGER NOT NULL,
    big_contract_address TEXT NOT NULL,
    FOREIGN KEY (network_of_major_token, big_contract_address) REFERENCES addresses(network_id, address),
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE legit_tokens
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    creator_network INTEGER NOT NULL,
    creator_address TEXT NOT NULL,
    FOREIGN KEY (creator_network, creator_address) REFERENCES addresses(network_id, address),
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);

CREATE TABLE holders
(
    network_id integer REFERENCES networks (network_id),
    address    TEXT NOT NULL,
    notes      TEXT NOT NULL,
    holder_type holder_types,
    FOREIGN KEY (network_id, address) REFERENCES addresses (network_id, address)
);
