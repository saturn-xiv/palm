-- migrate:up
CREATE TABLE daffodil_ledgers(
    id SERIAL PRIMARY KEY,
    owner_id INT NOT NULL,
    "uid" VARCHAR(36) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    summary VARCHAR(511) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_daffodil_ledgers_uid ON daffodil_ledgers("uid");
CREATE UNIQUE INDEX idx_daffodil_ledgers_owner_name ON daffodil_ledgers(owner_id, "name");
CREATE INDEX idx_daffodil_ledgers_name ON daffodil_ledgers("name");
CREATE TABLE daffodil_bills(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    ledger_id INT NOT NULL,
    summary VARCHAR(511) NOT NULL,
    price MONEY NOT NULL,
    currency CHAR(3) NOT NULL,
    merchant VARCHAR(64) NOT NULL,
    category VARCHAR(32) NOT NULL,
    paid_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    paid_by VARCHAR(32) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_daffodil_bills_currency ON daffodil_bills(currency);
CREATE INDEX idx_daffodil_bills_merchant ON daffodil_bills(merchant);
CREATE INDEX idx_daffodil_bills_summary ON daffodil_bills(summary);
CREATE INDEX idx_daffodil_bills_category ON daffodil_bills(category);
CREATE INDEX idx_daffodil_bills_paid_by ON daffodil_bills(paid_by);
CREATE TABLE daffodil_bills_history(
    id SERIAL PRIMARY KEY,    
    bill_id INT NOT NULL,
    user_id INT NOT NULL,
    summary VARCHAR(511) NOT NULL,
    price MONEY NOT NULL,
    currency CHAR(3) NOT NULL,
    merchant VARCHAR(64) NOT NULL,
    category VARCHAR(32) NOT NULL,
    paid_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    paid_by VARCHAR(32) NOT NULL,   
    reason VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_daffodil_bills_history_reason ON daffodil_bills_history(reason);
CREATE INDEX idx_daffodil_bills_history_currency ON daffodil_bills_history(currency);
CREATE INDEX idx_daffodil_bills_history_merchant ON daffodil_bills_history(merchant);
CREATE INDEX idx_daffodil_bill_historys_summary ON daffodil_bills_history(summary);
CREATE INDEX idx_daffodil_bills_history_category ON daffodil_bills_history(category);
CREATE INDEX idx_daffodil_bills_history_paid_by ON daffodil_bills_history(paid_by);
-- migrate:down
DROP TABLE daffodil_bills_history;
DROP TABLE daffodil_bills;
DROP TABLE daffodil_ledgers;
