-- Add migration script here
CREATE TABLE IF NOT EXISTS posts (
    id uuid PRIMARY KEY,
    creator_id uuid NOT NULL,
    description VARCHAR(255),
    video_url VARCHAR(255) NOT NULL,
    likes INT NOT NULL,
    paid BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,
    CONSTRAINT fk_creator_id
    FOREIGN KEY(creator_id) REFERENCES users(id)
    ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS post_images (
    id uuid PRIMARY KEY,
    post_id uuid NOT NULL,
    CONSTRAINT fk_post_id
    FOREIGN KEY(post_id) REFERENCES posts(id)
    ON DELETE NO ACTION
);