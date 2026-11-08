ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'normal' CHECK (role IN ('normal', 'forest', 'super'));
-- Owner decision: nobody loses access; he narrows accounts afterwards.
UPDATE users SET role = 'super';
ALTER TABLE users DROP COLUMN permissions, DROP COLUMN admin;
