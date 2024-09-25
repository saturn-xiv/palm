-- migrate:up
CREATE TABLE cms_pages(
    id SERIAL PRIMARY KEY,
    lang VARCHAR(15) NOT NULL,
    parent_id INT NOT NULL,
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
CREATE INDEX idx_cms_pages_title ON cms_pages(title);

-- migrate:down
DROP TABLE idx_cms_pages;
