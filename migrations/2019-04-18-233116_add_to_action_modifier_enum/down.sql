-- This file should undo anything in `up.sql`
CREATE TYPE action_modifier_new AS ENUM ('all', 'own');

ALTER TABLE permissions
  ALTER COLUMN modifier TYPE action_modifier_new
    USING (action_modifier::text::admin_level1_new);

-- and swap the types
DROP TYPE action_modifier;

ALTER TYPE action_modifier_new RENAME TO action_modifier;
