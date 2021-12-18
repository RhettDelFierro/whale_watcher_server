-- Add migration script here
ALTER TABLE legit_token_creators
    RENAME COLUMN network_of_major_token TO network_of_legit_token;

ALTER TABLE legit_token_creators
    RENAME COLUMN big_contract_address TO legit_contract_address;
