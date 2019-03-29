DROP TABLE events;
ALTER TABLE projects
DROP COLUMN event_id;

-- This file should undo anything in `up.sql`
