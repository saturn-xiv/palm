-- migrate:up
CREATE TABLE bookkeeper_ledgers(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    "uid" VARCHAR(36) NOT NULL,
    label VARCHAR(63) NOT NULL,
    memo VARCHAR(1023) NOT NULL,
    profile BYTEA NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_ledgers_uid ON bookkeeper_ledgers("uid");
CREATE INDEX idx_bookkeeper_ledgers_label ON bookkeeper_ledgers(label);

CREATE TABLE bookkeeper_accounts(
    id SERIAL PRIMARY KEY,    
    ledger_id INTEGER NOT NULL,
    parent_id INTEGER,    
    label VARCHAR(63) NOT NULL,
    memo VARCHAR(1023) NOT NULL,    
    currency_id INTEGER NOT NULL,
    "type" VARCHAR(15) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_accounts_ledger_label ON bookkeeper_accounts(ledger_id, label);
CREATE INDEX idx_bookkeeper_accounts_label ON bookkeeper_accounts(label);
CREATE INDEX idx_bookkeeper_accounts_type ON bookkeeper_accounts("type");

CREATE TABLE bookkeeper_categories(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,
    parent_id INTEGER,    
    label VARCHAR(63) NOT NULL, 
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_categories_ledger_label ON bookkeeper_categories(ledger_id, label);
CREATE INDEX idx_bookkeeper_categories_label ON bookkeeper_categories(label);

CREATE TABLE bookkeeper_merchants(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,    
    label VARCHAR(63) NOT NULL,
    memo VARCHAR(1023) NOT NULL,    
    address INTEGER,
    contact INTEGER,    
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_merchants_ledger_label ON bookkeeper_merchants(ledger_id, label);
CREATE INDEX idx_bookkeeper_merchants_label ON bookkeeper_merchants(label);
CREATE INDEX idx_bookkeeper_merchants_memo ON bookkeeper_merchants(memo);

CREATE TABLE bookkeeper_transactions(
    id SERIAL PRIMARY KEY,
    "uid" VARCHAR(36) NOT NULL,
    ledger_id INTEGER NOT NULL,    
    memo VARCHAR(1023) NOT NULL,    
    traded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    timezone VARCHAR(31) NOT NULL DEFAULT 'UTC',   
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_transactions_uid ON bookkeeper_transactions("uid");
CREATE INDEX idx_bookkeeper_transactions_memo ON bookkeeper_transactions(memo);
CREATE INDEX idx_bookkeeper_transactions_timezone ON bookkeeper_transactions(timezone);

CREATE TABLE bookkeeper_entries(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,
    sn CHAR(18) NOT NULL,
    transaction_id INTEGER NOT NULL,
    from_account_id INTEGER NOT NULL,
    to_account_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    merchant_id INTEGER NOT NULL,
    currency_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    memo VARCHAR(1023) NOT NULL,    
    traded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    timezone VARCHAR(31) NOT NULL DEFAULT 'UTC',
    "status" VARCHAR(31) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_entries_ledger_sn ON bookkeeper_entries(ledger_id, sn);
CREATE INDEX idx_bookkeeper_entries_memo ON bookkeeper_entries(memo);
CREATE INDEX idx_bookkeeper_entries_timezone ON bookkeeper_entries(timezone);
CREATE INDEX idx_bookkeeper_entries_status ON bookkeeper_entries("status");

CREATE TABLE bookkeeper_statements(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,    
    account_id INTEGER NOT NULL,
    transaction_id INTEGER NOT NULL,
    entry_id INTEGER NOT NULL,    
    currency_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    "type" VARCHAR(7) NOT NULL, 
    opening_balance INTEGER NOT NULL,
    closing_balance INTEGER NOT NULL,     
    traded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    timezone VARCHAR(31) NOT NULL DEFAULT 'UTC',   
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_statements_ledger_account_entry ON bookkeeper_statements(ledger_id, account_id, entry_id);
CREATE INDEX idx_bookkeeper_statements_type ON bookkeeper_statements("type");

CREATE TABLE bookkeeper_logs(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    username VARCHAR(511) NOT NULL,
    action VARCHAR(31) NOT NULL,
    memo TEXT NOT NULL,
    reason VARCHAR(255),
    ip VARCHAR(45) NOT NULL,       
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_bookkeeper_logs_action ON bookkeeper_logs(action);
CREATE INDEX idx_bookkeeper_logs_ip ON bookkeeper_logs(ip);
CREATE INDEX idx_bookkeeper_logs_username ON bookkeeper_logs(username);
CREATE INDEX idx_bookkeeper_logs_reason ON bookkeeper_logs(reason) WHERE reason IS NOT NULL;

-- migrate:down
DROP TABLE bookkeeper_logs;
DROP TABLE bookkeeper_statements;
DROP TABLE bookkeeper_entries;
DROP TABLE bookkeeper_transactions;
DROP TABLE bookkeeper_merchants;
DROP TABLE bookkeeper_categories;
DROP TABLE bookkeeper_accounts;
DROP TABLE bookkeeper_ledgers;
