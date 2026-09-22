-- Add migration script here
ALTER TABLE identity.users ADD COLUMN role VARCHAR(50);
