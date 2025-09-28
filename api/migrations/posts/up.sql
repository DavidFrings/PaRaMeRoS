CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL,
    name VARCHAR NOT NULL,
    heading VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    media_type VARCHAR,
    media_name VARCHAR,
    media_creator VARCHAR,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP

    CONSTRAINT media_fields CHECK (
        (
            media_type IS NULL
            AND media_name IS NULL
            AND media_creator IS NULL
        ) OR (
            media_type IS NOT NULL
            AND media_name IS NOT NULL
            AND media_creator IS NOT NULL
        )
    )
);