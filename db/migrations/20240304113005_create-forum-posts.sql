-- migrate:up
CREATE TABLE forum_posts(
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL,
    topic_id BIGINT NOT NULL,
    post_id BIGINT,
    lang VARCHAR(8) NOT NULL,
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_forum_posts_lang ON forum_posts(lang);
CREATE INDEX idx_forum_posts_body_editor ON forum_posts(body_editor);
-- migrate:down
DROP TABLE forum_posts;
