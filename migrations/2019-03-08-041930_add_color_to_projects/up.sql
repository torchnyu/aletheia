ALTER TABLE projects
ADD COLUMN color VARCHAR;
UPDATE projects SET color = '#444b6e';
ALTER TABLE projects
ALTER COLUMN color SET NOT NULL;
