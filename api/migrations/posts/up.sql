CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL,
    name VARCHAR NOT NULL,
    heading VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    media_type VARCHAR,
    media_name VARCHAR,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);