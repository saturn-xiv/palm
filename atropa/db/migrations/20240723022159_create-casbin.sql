-- migrate:up
CREATE TABLE casbin_rule(
    id SERIAL PRIMARY KEY,
    ptype VARCHAR(255) NOT NULL DEFAULT '',
    v0 VARCHAR(255) NOT NULL DEFAULT '',
    v1 VARCHAR(255) NOT NULL DEFAULT '',
    v2 VARCHAR(255) NOT NULL DEFAULT '',
    v3 VARCHAR(255) NOT NULL DEFAULT '',
    v4 VARCHAR(255) NOT NULL DEFAULT '',
    v5 VARCHAR(255) NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX idx_casbin_rule ON casbin_rule(ptype, v0, v1, v2, v3, v4, v5);

-- migrate:down
DROP TABLE casbin_rule;
