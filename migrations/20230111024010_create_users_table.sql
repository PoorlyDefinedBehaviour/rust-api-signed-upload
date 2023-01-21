-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    accepted_terms_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    banned_at TIMESTAMP WITH TIME ZONE ,
    deleted_at TIMESTAMP WITH TIME ZONE 
);

CREATE TABLE IF NOT EXISTS user_tags (
    id uuid PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id uuid NOT NULL,
    CONSTRAINT fk_user_id
    FOREIGN KEY(user_id) REFERENCES users(id)
    ON DELETE NO ACTION
);