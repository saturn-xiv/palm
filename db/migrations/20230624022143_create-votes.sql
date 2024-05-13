-- migrate:up
CREATE TABLE vote_items(
    id BIGSERIAL PRIMARY KEY,
    star_ INT NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_vote_items ON vote_items(resource_type, resource_id);
CREATE INDEX idx_vote_items_resource_type ON vote_items(resource_type);
CREATE TABLE vote_logs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    ip VARCHAR(45) NOT NULL,
    star_ INT NOT NULL,
    comment TEXT NOT NULL,
    comment_editor INT NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    "status" INT NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_vote_logs ON vote_logs(user_id, resource_type, resource_id);
CREATE INDEX idx_vote_logs_ip ON vote_logs(ip);
CREATE INDEX idx_vote_logs_resource_type ON vote_logs(resource_type);
-- migrate:down
DROP TABLE vote_logs;
DROP TABLE vote_items;
