-- migrate:up
CREATE TABLE cms_pages(
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL,
    slug VARCHAR(63) NOT NULL,
    title VARCHAR(255) NOT NULL,
    lang VARCHAR(8) NOT NULL,
    summary VARCHAR(511) NOT NULL,
    body TEXT NOT NULL,
    body_editor INT NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_cms_pages_slug_lang ON cms_pages(slug, lang);
CREATE INDEX idx_cms_pages_slug ON cms_pages(slug);
CREATE INDEX idx_cms_pages_lang ON cms_pages(lang);
CREATE INDEX idx_cms_pages_title ON cms_pages(title);
CREATE INDEX idx_cms_pages_summary ON cms_pages(summary);
-- migrate:down
DROP TABLE cms_pages;
