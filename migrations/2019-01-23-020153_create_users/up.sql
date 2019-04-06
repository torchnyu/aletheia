CREATE SEQUENCE users_username_seq;
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  display_name VARCHAR NOT NULL DEFAULT 'user' || nextval('users_username_seq'),
  email VARCHAR NOT NULL
)
