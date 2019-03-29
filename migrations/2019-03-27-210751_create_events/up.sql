CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  description VARCHAR
);

ALTER TABLE projects
ADD COLUMN event_id INTEGER NOT NULL;

ALTER TYPE resource rename TO _resource;
CREATE TYPE resource AS ENUM (
  'project',
  'submission',
  'user',
  'permission',
  'role',
  'user_role',
  'event',
  'user_event'
);

ALTER TABLE permissions RENAME COLUMN resource_name TO _resource_name;

ALTER TABLE permissions ADD resource_name resource NOT NULL DEFAULT 'project';

UPDATE permissions SET resource_name = _resource_name::text::resource;

ALTER TABLE permissions DROP COLUMN _resource_name;
DROP TYPE _resource;
-- Your SQL goes here
