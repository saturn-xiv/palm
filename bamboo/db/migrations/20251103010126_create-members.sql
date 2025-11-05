-- migrate:up
CREATE TABLE members(
    id SERIAL PRIMARY KEY,
    sn VARCHAR(31) NOT NULL,
    name VARCHAR(31) NOT NULL,
    wifi_password VARCHAR(255) NOT NULL,
    profile BYTEA NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_members_sn ON members(sn);
CREATE INDEX idx_members_name ON members(name);
-- migrate:down
DROP TABLE members;
