-- migrate:up
CREATE TABLE favorites(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    "subject" VARCHAR(127) NOT NULL,
    "url" VARCHAR(255) NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_favorites_user_url ON favorites(user_id, "url");
CREATE INDEX idx_favorites_url ON favorites("url");
CREATE INDEX idx_favorites_subject ON favorites("subject");
CREATE INDEX idx_favorites_resource_type ON favorites(resource_type);
-- migrate:down
DROP TABLE favorites;
