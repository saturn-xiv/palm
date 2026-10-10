-- migrate:up
CREATE TABLE wechat_mini_program_users(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    uid VARCHAR(36) NOT NULL,
    union_id VARCHAR(127) NOT NULL,
    app_id VARCHAR(63) NOT NULL,
    open_id VARCHAR(63) NOT NULL,
    nickname VARCHAR(63),
    avatar_url VARCHAR(127),
    session_key VARCHAR(127) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at  TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_wechat_mini_program_users_uid ON wechat_mini_program_users(uid);
CREATE UNIQUE INDEX idx_wechat_mini_program_users_app_open ON wechat_mini_program_users(app_id, open_id);
CREATE INDEX idx_wechat_mini_program_users_union ON wechat_mini_program_users(union_id);
CREATE INDEX idx_wechat_mini_program_users_nickname ON wechat_mini_program_users(nickname) WHERE nickname IS NOT NULL;

-- migrate:down
DROP TABLE wechat_mini_program_users;
