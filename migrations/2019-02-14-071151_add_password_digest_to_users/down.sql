ALTER TABLE users
DROP COLUMN password_digest,
DROP COLUMN salt;
-- This file should undo anything in `up.sql`
