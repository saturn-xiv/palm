-- migrate:up
CREATE TABLE casbin_rule (
    id INT NOT NULL AUTO_INCREMENT,
    ptype VARCHAR(127) NOT NULL,
    v0 VARCHAR(127) NOT NULL,
    v1 VARCHAR(127) NOT NULL,
    v2 VARCHAR(127) NOT NULL DEFAULT '',
    v3 VARCHAR(127) NOT NULL DEFAULT '',
    v4 VARCHAR(127) NOT NULL DEFAULT '',
    v5 VARCHAR(127) NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX idx_casbin_rule ON casbin_rule(ptype, v0, v1, v2, v3, v4, v5);
CREATE INDEX idx_casbin_rule_ptype ON casbin_rule(ptype);
CREATE INDEX idx_casbin_rule_0 ON casbin_rule(v0);
CREATE INDEX idx_casbin_rule_1 ON casbin_rule(v1);
CREATE INDEX idx_casbin_rule_2 ON casbin_rule(v2);
CREATE INDEX idx_casbin_rule_3 ON casbin_rule(v3);
CREATE INDEX idx_casbin_rule_4 ON casbin_rule(v4);
CREATE INDEX idx_casbin_rule_5 ON casbin_rule(v5);
-- migrate:down
DROP TABLE casbin_rule;
