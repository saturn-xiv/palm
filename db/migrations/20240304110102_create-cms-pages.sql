-- migrate:up
CREATE TABLE cms_pages(
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL,
    slug VARCHAR(63) NOT NULL,
    title VARCHAR(255) NOT NULL,
    lang VARCHAR(8) NOT NULL,
    summary VARCHAR(511) NOT NULL,
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
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
CREATE INDEX idx_cms_pages_body_editor ON cms_pages(body_editor);
-- migrate:down
DROP TABLE cms_pages;
