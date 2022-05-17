CREATE TABLE settings (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    'key' VARCHAR(255) NOT NULL,
    salt BYTEA,
    value BYTEA NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_settings_user_id_key ON settings(user_id, 'key')
WHERE user_id IS NOT NULL;
CREATE UNIQUE INDEX idx_settings_key ON settings('key')
WHERE user_id IS NULL;
