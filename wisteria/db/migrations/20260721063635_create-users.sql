-- migrate:up
CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    uid VARCHAR(36) NOT NULL,
    name VARCHAR(31) NOT NULL,
    avatar VARCHAR(127),
    lang VARCHAR(7) NOT NULL DEFAULT 'en-US',
    timezone VARCHAR(31) NOT NULL DEFAULT 'UTC',
    sign_in_count INTEGER NOT NULL DEFAULT 0,
    current_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    current_sign_in_ip VARCHAR(45),
    last_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    last_sign_in_ip VARCHAR(45),
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_users ON users(uid);
CREATE INDEX idx_users_name ON users(name);
CREATE INDEX idx_users_lang ON users(lang);
CREATE INDEX idx_users_timezone ON users(timezone);
CREATE INDEX idx_users_name ON users(name) WHERE name IS NOT NULL;
CREATE INDEX idx_users_current_sign_in_ip ON users(current_sign_in_ip) WHERE current_sign_in_ip IS NOT NULL;
CREATE INDEX idx_users_last_sign_in_ip ON users(last_sign_in_ip) WHERE last_sign_in_ip IS NOT NULL;

CREATE TABLE user_contacts(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code VARCHAR(15) NOT NULL,
    value bytea NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_user_contacts ON user_contacts(user_id, code);
CREATE INDEX idx_user_contacts_code ON user_contacts(code);

CREATE TABLE user_bans(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    creator_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ip VARCHAR(45) NOT NULL,
    reason TEXT NOT NULL,
    expired_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_user_bans_ip ON user_bans(ip);

CREATE TABLE logs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plugin VARCHAR(31) NOT NULL,
    level VARCHAR(7) NOT NULL,
    ip VARCHAR(45) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_logs_plugin ON logs(plugin);
CREATE INDEX idx_logs_level ON logs(level);
CREATE INDEX idx_logs_ip ON logs(ip);

-- migrate:down
DROP TABLE logs;
DROP TABLE user_bans;
DROP TABLE user_contacts;
DROP TABLE users;
