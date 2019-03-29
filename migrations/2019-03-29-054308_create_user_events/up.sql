CREATE TABLE user_events (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  event_id INTEGER REFERENCES events(id) NOT NULL
)
-- Your SQL goes here
