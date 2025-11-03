-- migrate:up
CREATE TABLE settings(
    id SERIAL PRIMARY KEY,
    "key" VARCHAR(255) NOT NULL,
    salt BYTEA,
    "value" BYTEA NOT NULL,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- migrate:down
DROP TABLE settings;
