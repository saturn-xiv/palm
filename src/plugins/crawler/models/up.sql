CREATE TABLE crawler_sites(
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    url VARCHAR(255) NOT NULL,
    ttl INTEGER NOT NULL DEFAULT 86400,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_crawler_sites_url ON crawler_sites(url);

CREATE INDEX idx_crawler_sites_name ON crawler_sites(name);

CREATE TABLE crawler_logs(
    id SERIAL PRIMARY KEY,
    url VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crawler_logs_url ON crawler_logs(url);
