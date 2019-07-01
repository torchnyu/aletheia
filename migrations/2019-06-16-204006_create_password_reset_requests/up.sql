CREATE TABLE password_reset_requests (
  id VARCHAR PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
