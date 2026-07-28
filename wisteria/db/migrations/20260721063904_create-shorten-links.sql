-- migrate:up
CREATE TABLE shorten_links(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(127) NOT NULL,
    url VARCHAR(127) NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_shorten_links_url ON shorten_links(url);
CREATE INDEX idx_shorten_links_title ON shorten_links(title);

-- migrate:down
DROP TABLE shorten_links;
