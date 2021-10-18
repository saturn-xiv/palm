CREATE TABLE crawler_sites(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    url VARCHAR(255) NOT NULL,
    ttl INTEGER NOT NULL DEFAULT 15,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_crawler_sites_url ON crawler_sites(url);

CREATE UNIQUE INDEX idx_crawler_sites_name ON crawler_sites(name);

CREATE TABLE crawler_logs(
    id BIGSERIAL PRIMARY KEY,
    site_id BIGINT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
