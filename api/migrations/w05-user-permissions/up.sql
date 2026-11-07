ALTER TABLE users ADD COLUMN permissions TEXT[] NOT NULL DEFAULT '{}';
-- Writes were never checked before, so existing non-admins keep every section (the owner narrows later).
UPDATE users SET permissions = ARRAY['logbuch', 'team', 'coaches', 'forest-project', 'tournament'] WHERE NOT admin;
