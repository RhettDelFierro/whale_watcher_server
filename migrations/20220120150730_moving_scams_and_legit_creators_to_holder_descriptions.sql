-- Add migration script here
BEGIN;
    Insert into holder_descriptions (network_id, holder_address, contract_address, notes, address_types)
    Select network_of_scammed_token, address, scammed_contract_address, notes, ARRAY['token_creator', 'scammer']
    from scam_token_creators;

    Insert into holder_descriptions (network_id, holder_address, contract_address, notes, address_types)
    Select network_of_legit_token, address, legit_contract_address, notes, ARRAY['token_creator', 'legit']
    from legit_token_creators;
COMMIT;