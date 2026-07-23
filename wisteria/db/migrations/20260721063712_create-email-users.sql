-- migrate:up
CREATE TABLE email_users(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(63) NOT NULL,
    password VARCHAR(127) NOT NULL,
    value BYTEA NOT NULL,
    associated_data BYTEA,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_settings_key ON settings(key) WHERE user_id IS NULL;

-- migrate:down
DROP TABLE email_users;
