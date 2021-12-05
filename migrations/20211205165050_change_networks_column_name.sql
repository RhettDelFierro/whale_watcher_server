-- Add migration script here
ALTER TABLE networks RENAME COLUMN token_name TO network_name;