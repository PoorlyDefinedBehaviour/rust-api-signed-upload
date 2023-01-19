-- Add migration script here
CREATE TABLE timeline (
    id uuid PRIMARY KEY,
    content_creator_username VARCHAR(255) NOT NULL,
    content_creator_avatar_url VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    media_url VARCHAR(255) NOT NULL,
    likes INT NOT NULL
);
