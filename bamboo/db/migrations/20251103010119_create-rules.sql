-- migrate:up
CREATE TABLE rules(
    id SERIAL PRIMARY KEY,
    name VARCHAR(31) NOT NULL,
    content BYTEA NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_rules_name ON rules(name);
-- migrate:down
DROP TABLE rules;
