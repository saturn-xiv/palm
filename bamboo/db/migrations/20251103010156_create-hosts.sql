-- migrate:up
CREATE TABLE hosts(
    id SERIAL PRIMARY KEY,
    member_id INTEGER,
    name VARCHAR(63),
    mac CHAR(17) NOT NULL,
    ip VARCHAR(39) NOT NULL,
    fixed BOOLEAN NOT NULL DEFAULT FALSE,
    memo TEXT NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_hosts_mac ON hosts(mac);
CREATE INDEX idx_hosts_name ON hosts(name)
WHERE name IS NOT NULL;
CREATE INDEX idx_hosts_ip ON hosts(ip);
CREATE TABLE hosts_rules(
    id SERIAL PRIMARY KEY,
    host_id INTEGER NOT NULL,
    rule_id INTEGER NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- migrate:down
DROP TABLE hosts_rules;
DROP TABLE hosts;
