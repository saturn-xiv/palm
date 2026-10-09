-- migrate:up
CREATE TABLE wechat_oauth2_users(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    union_id VARCHAR(127) NOT NULL,
    app_id VARCHAR(63) NOT NULL,
    open_id VARCHAR(63) NOT NULL,
    nickname VARCHAR(63) NOT NULL,
    head_img_url VARCHAR(127),
    privilege JSONB NOT NULL,
    access_token VARCHAR(63) NOT NULL,
    expires_in TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    refresh_token  VARCHAR(63) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at  TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_wechat_oauth2_users_app_open ON wechat_oauth2_users(app_id, open_id);
CREATE INDEX idx_wechat_oauth2_users_union ON wechat_oauth2_users(union_id);
CREATE INDEX idx_wechat_oauth2_users_nickname ON wechat_oauth2_users(nickname);

-- migrate:down
DROP TABLE wechat_oauth2_users;
