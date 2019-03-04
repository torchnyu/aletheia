CREATE TABLE submissions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  project_id INTEGER REFERENCES projects(id) NOT NULL
)
