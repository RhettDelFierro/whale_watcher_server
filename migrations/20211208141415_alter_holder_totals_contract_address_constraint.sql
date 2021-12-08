-- Add migration script here
BEGIN;
    UPDATE holder_totals
        SET contract_address = 'unavailable'
        WHERE contract_address IS NULL;

    ALTER TABLE holder_totals
        ALTER COLUMN contract_address SET NOT NULL;

    ALTER TABLE holder_totals
        ADD CONSTRAINT ht_contract_address_fk
            FOREIGN KEY (network_id, contract_address)
                REFERENCES addresses(network_id, address);
COMMIT;