-- Add migration script here
ALTER TABLE holder_descriptions DROP COLUMN address_type;

ALTER TABLE holder_descriptions ADD COLUMN address_types text[];