ALTER TABLE users
ADD COLUMN password_digest VARCHAR NOT NULL,
ADD COLUMN salt            VARCHAR NOT NULL;
-- Your SQL goes here
