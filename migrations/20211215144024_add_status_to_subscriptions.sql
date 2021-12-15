-- Add migration script here
ALTER TABLE subscriptions ADD COLUMN status TEXT NULL;

ALTER TABLE scam_token_creators DROP COLUMN network_id;
ALTER TABLE scam_tokens DROP COLUMN network_id;

ALTER TABLE legit_token_creators DROP COLUMN network_id;
ALTER TABLE legit_tokens DROP COLUMN network_id;