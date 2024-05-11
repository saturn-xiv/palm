-- migrate:up
CREATE TABLE google_users(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    email VARCHAR(255),
    email_verified BOOLEAN NOT NULL,
    name VARCHAR(63),
    picture VARCHAR(255),
    sub VARCHAR(127) NOT NULL,
    code BYTEA NOT NULL,
    token VARCHAR(127) NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_google_users_sub ON google_users("sub");
CREATE INDEX idx_google_users_email ON google_users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_google_users_name ON google_users(name) WHERE name IS NOT NULL;
-- migrate:down
DROP TABLE google_users;
