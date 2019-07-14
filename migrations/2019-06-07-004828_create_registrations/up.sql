CREATE TABLE registrations (
  id SERIAL PRIMARY KEY,
  application JSONB NOT NULL,
  confirmation JSONB
)
