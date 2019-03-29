CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  description VARCHAR
);

ALTER TABLE projects
ADD COLUMN event_id INTEGER NOT NULL;
-- Your SQL goes here
