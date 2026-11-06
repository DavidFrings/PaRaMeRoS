CREATE TABLE tournament (
    id INT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    name VARCHAR NOT NULL,
    place VARCHAR,
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL
);
