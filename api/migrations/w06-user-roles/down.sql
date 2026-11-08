ALTER TABLE users
    ADD COLUMN admin BOOL NOT NULL DEFAULT false,
    ADD COLUMN permissions TEXT[] NOT NULL DEFAULT '{}';
UPDATE users SET
    admin = (role = 'super'),
    permissions = CASE role
        WHEN 'normal' THEN ARRAY['logbuch', 'tournament', 'coaches', 'team']
        ELSE ARRAY['logbuch', 'tournament', 'coaches', 'team', 'forest-project']
    END;
ALTER TABLE users DROP COLUMN role;
