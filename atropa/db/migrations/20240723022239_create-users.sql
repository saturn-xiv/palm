-- migrate:up
CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    uid VARCHAR(36) NOT NULL,    
    name VARCHAR(63),
    avatar VARCHAR(255),
    lang VARCHAR(15) NOT NULL DEFAULT 'en-US',
    timezone VARCHAR(31) NOT NULL DEFAULT 'UTC',
    sign_in_count INT NOT NULL DEFAULT 0,
    current_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    current_sign_in_ip VARCHAR(45),
    last_sign_in_at TIMESTAMP WITHOUT TIME ZONE,
    last_sign_in_ip VARCHAR(45),
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_users_uid ON users(uid);
CREATE INDEX idx_users_name ON users(name) WHERE name IS NOT NULL;
CREATE INDEX idx_users_lang ON users(lang);
CREATE INDEX idx_users_timezone ON users(timezone);
CREATE INDEX idx_users_current_sign_in_ip ON users(current_sign_in_ip) WHERE current_sign_in_ip IS NOT NULL;
CREATE INDEX idx_users_last_sign_in_ip ON users(last_sign_in_ip) WHERE last_sign_in_ip IS NOT NULL;


CREATE TABLE logs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    plugin VARCHAR(31) NOT NULL,
    ip VARCHAR(45) NOT NULL,
    level VARCHAR(8) NOT NULL,
    resource_type VARCHAR(127) NOT NULL,
    resource_id BIGINT,
    message TEXT NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_logs_plugin ON logs(plugin);
CREATE INDEX idx_logs_ip ON logs(ip);
CREATE INDEX idx_logs_level ON logs(level);
CREATE INDEX idx_logs_resource_type ON logs(resource_type);


CREATE TABLE user_sessions(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    uid VARCHAR(36) NOT NULL, 
    resource_type VARCHAR(127) NOT NULL,
    resource_id BIGINT NOT NULL,
    ip VARCHAR(45) NOT NULL,
    expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_user_sessions_uid ON user_sessions(uid);
CREATE INDEX idx_user_sessions_ip ON user_sessions(ip);
CREATE INDEX idx_user_sessions_resource_type ON user_sessions(resource_type);


-- migrate:down
DROP TABLE user_sessions;
DROP TABLE logs;
DROP TABLE users;
