ALTER TABLE users ADD CONSTRAINT unique_display_name UNIQUE (display_name);
ALTER TABLE users ADD CONSTRAINT unique_email UNIQUE (email);
-- Your SQL goes here
