-- migrate:up
CREATE TABLE forum_topics(
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL,
    slug VARCHAR(63) NOT NULL,
    "subject" VARCHAR(255) NOT NULL,
    lang VARCHAR(8) NOT NULL,
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_forum_topics_slug ON forum_topics(slug);
CREATE INDEX idx_forum_topics_lang ON forum_topics(lang);
CREATE INDEX idx_forum_topics_subject ON forum_topics("subject");
CREATE INDEX idx_forum_topics_body_editor ON forum_topics(body_editor);
-- migrate:down
DROP TABLE forum_topics;
