CREATE TABLE crawler_logs(
    id BIGSERIAL PRIMARY KEY,
    url VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_crawler_logs_url ON crawler_logs(url);
