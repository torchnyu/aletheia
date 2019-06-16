CREATE TABLE password_reset_requests (
  id VARCHAR PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  user_id INTEGER REFERENCES users(id) NOT NULL
);
