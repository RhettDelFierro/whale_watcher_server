-- Add migration script here
CREATE TABLE holders(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  holder_address TEXT NOT NULL UNIQUE
);