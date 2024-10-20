-- migrate:up
CREATE TABLE cms_pages(
    id SERIAL PRIMARY KEY,
    lang VARCHAR(15) NOT NULL,
    slug VARCHAR(31) NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    body_editor VARCHAR(15) NOT NULL,
    template VARCHAR(15) NOT NULL,    
    profile BYTEA NOT NULL,
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_cms_pages_lang_slug ON cms_pages(lang, slug);
CREATE INDEX idx_cms_pages_title ON cms_pages(title);
CREATE INDEX idx_cms_pages_lang ON cms_pages(lang);
CREATE INDEX idx_cms_pages_slug ON cms_pages(slug);
CREATE INDEX idx_cms_pages_status ON cms_pages(status);


-- migrate:down
DROP TABLE idx_cms_pages;
