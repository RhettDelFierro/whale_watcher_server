-- Add migration script here
ALTER TABLE addresses ADD UNIQUE (address_id);
ALTER TABLE token_names ADD UNIQUE (token_name_id);
ALTER TABLE networks ADD UNIQUE (network_id);

CREATE TABLE address_token_names
(
    address_id    integer not null,
    token_name_id integer not null,
    FOREIGN KEY (address_id) REFERENCES addresses (address_id),
    FOREIGN KEY (token_name_id) REFERENCES token_names (token_name_id)
);