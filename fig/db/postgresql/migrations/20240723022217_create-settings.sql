-- migrate:up
CREATE TABLE settings(
    id SERIAL PRIMARY KEY,
    user_id INT,    
    key VARCHAR(255) NOT NULL,
    value BYTEA NOT NULL,
    nonce BYTEA,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_settings_key ON settings(key) WHERE user_id IS NULL;
CREATE UNIQUE INDEX idx_settings_key_user ON settings(key, user_id) WHERE user_id IS NOT NULL;

-- migrate:down
DROP TABLE settings;
