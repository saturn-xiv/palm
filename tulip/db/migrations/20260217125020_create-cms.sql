-- migrate:up
CREATE TABLE cms_pages(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title VARCHAR(127) NOT NULL,
    summary VARCHAR(1023) NOT NULL,
    body TEXT NOT NULL,
    body_editor INT NOT NULL,
    permalink VARCHAR(63) NOT NULL,
    location VARCHAR(255),
    published_at TIMESTAMP WITHOUT TIME ZONE,    
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_cms_pages_permalink ON cms_pages(permalink);
CREATE INDEX idx_cms_pages_title ON cms_pages(title);
CREATE INDEX idx_cms_pages_summary ON cms_pages(summary);
CREATE INDEX idx_cms_pages_location ON cms_pages(location) WHERE location IS NOT NULL;

-- migrate:down
DROP TABLE cms_pages;
