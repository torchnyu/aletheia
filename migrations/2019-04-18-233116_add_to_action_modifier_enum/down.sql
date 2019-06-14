-- This file should undo anything in `up.sql`
CREATE TYPE action_modifier_new AS ENUM ('all', 'own');

-- Remove all instances of 'one'
UPDATE permissions SET modifier = array_remove(modifier, 'one'::action_modifier) WHERE modifier @> ARRAY['one']::action_modifier[];

ALTER TABLE permissions
  ALTER COLUMN modifier TYPE action_modifier_new[]
    USING (modifier::text::action_modifier_new[]);

-- and swap the types
DROP TYPE action_modifier;

ALTER TYPE action_modifier_new RENAME TO action_modifier;
