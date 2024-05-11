-- migrate:up
CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    "uid" VARCHAR(36) NOT NULL,
    lang VARCHAR(8) NOT NULL DEFAULT 'en-US',
    timezone VARCHAR(32) NOT NULL DEFAULT 'UTC',    
    sign_in_count INT NOT NULL DEFAULT 0,
    current_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    current_sign_in_ip VARCHAR(45),
    last_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    last_sign_in_ip VARCHAR(45),    
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_users_uid ON users("uid");
CREATE INDEX idx_users_lang ON users(lang);
CREATE INDEX idx_users_timezone ON users(timezone);
CREATE INDEX idx_users_current_sign_in_ip ON users(current_sign_in_ip)
WHERE current_sign_in_ip IS NOT NULL;
CREATE INDEX idx_users_last_sign_in_ip ON users(last_sign_in_ip)
WHERE last_sign_in_ip IS NOT NULL;
CREATE TABLE user_contacts(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    "key" VARCHAR(255) NOT NULL,
    "value" BYTEA NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_user_contacts_user_key ON user_contacts(user_id, "key");
CREATE INDEX idx_user_contacts_key ON user_contacts("key");
CREATE TABLE logs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    plugin VARCHAR(15) NOT NULL,
    "level" VARCHAR(15) NOT NULL,
    ip VARCHAR(45) NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT,
    "message" TEXT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_logs_ip ON logs(ip);
CREATE INDEX idx_logs_plugin ON logs(plugin);
CREATE INDEX idx_logs_level ON logs("level");
CREATE INDEX idx_logs_resource_type ON logs(resource_type);
CREATE TABLE user_bans(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    ip VARCHAR(45) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    expired_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    creator_id BIGINT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_user_bans_ip ON user_bans(ip);
CREATE INDEX idx_user_bans_reason ON user_bans(reason);
CREATE TABLE user_sessions(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    "uid" VARCHAR(36) NOT NULL,
    provider_type VARCHAR(31) NOT NULL,
    provider_id BIGINT NOT NULL,
    ip VARCHAR(45) NOT NULL,
    expired_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_user_sessions_uid ON user_sessions("uid");
CREATE INDEX idx_user_sessions_provider_type ON user_sessions(provider_type);
CREATE INDEX idx_user_sessions_ip ON user_sessions(ip);
-- migrate:down
DROP TABLE user_sessions;
DROP TABLE user_bans;
DROP TABLE logs;
DROP TABLE user_contacts;
DROP TABLE users;
