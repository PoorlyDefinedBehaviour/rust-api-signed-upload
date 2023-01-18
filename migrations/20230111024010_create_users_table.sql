-- Add migration script here
CREATE TABLE users (
    id uuid PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    accepted_terms_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,
    banned_at TIMESTAMP,
    deleted_at TIMESTAMP
);
