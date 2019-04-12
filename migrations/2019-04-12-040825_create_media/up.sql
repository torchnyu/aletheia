CREATE TABLE media (
  id SERIAL PRIMARY KEY,
  file_name VARCHAR NOT NULL,
  project_id INTEGER REFERENCES projects(id)
);
-- Your SQL goes here
