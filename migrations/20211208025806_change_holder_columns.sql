-- Add migration script here
ALTER TABLE holder_totals
    RENAME COLUMN address TO holder_address;

ALTER TABLE holder_totals
    ADD contract_address TEXT;
