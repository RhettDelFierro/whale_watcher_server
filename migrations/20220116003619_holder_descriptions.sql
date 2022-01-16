-- Add migration script here
CREATE TABLE holder_descriptions
(
    network_id       integer REFERENCES networks (network_id),
    holder_address   TEXT NOT NULL,
    contract_address TEXT NOT NULL,
    notes            TEXT,
    address_type     TEXT NOT NULL
);