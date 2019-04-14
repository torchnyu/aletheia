ALTER TABLE media
ADD COLUMN user_id INTEGER REFERENCES users(id);
-- Your SQL goes here
