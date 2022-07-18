-- Your SQL goes here
CREATE TABLE usr (
    id SERIAL PRIMARY KEY,
    name varchar not null,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('usr');
