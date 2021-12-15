-- Add migration script here
ALTER TABLE scam_tokens ALTER COLUMN notes DROP NOT NULL;
ALTER TABLE scam_token_creators ALTER COLUMN notes DROP NOT NULL;
ALTER TABLE legit_tokens ALTER COLUMN notes DROP NOT NULL;
ALTER TABLE legit_token_creators ALTER COLUMN notes DROP NOT NULL;