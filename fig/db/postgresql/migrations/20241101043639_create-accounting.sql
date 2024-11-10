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
CREATE INDEX idx_bookkeeper_categories_label ON bookkeeper_categories(label);

CREATE TABLE bookkeeper_merchants(
    id SERIAL PRIMARY KEY,
    ledger_id INTEGER NOT NULL,    
    label VARCHAR(63) NOT NULL,
    memo VARCHAR(1023) NOT NULL,
    contact VARCHAR(127),
    addresses BYTEA NOT NULL,
    phones BYTEA NOT NULL,
    maps BYTEA NOT NULL,   
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_bookkeeper_merchants_label ON bookkeeper_merchants(label);
CREATE INDEX idx_bookkeeper_merchants_memo ON bookkeeper_merchants(memo);
CREATE INDEX idx_bookkeeper_merchants_contact ON bookkeeper_merchants(contact) WHERE contact IS NOT NULL;

CREATE TABLE bookkeeper_transactions(
    id SERIAL PRIMARY KEY,
    "uid" VARCHAR(36) NOT NULL,
    ledger_id INTEGER NOT NULL,
    memo VARCHAR(1023) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_bookkeeper_transactions_uid ON bookkeeper_transactions("uid");
CREATE INDEX idx_bookkeeper_transactions_memo ON bookkeeper_transactions(memo);

CREATE TABLE bookkeeper_entries(
    id SERIAL PRIMARY KEY,
    transaction_id INTEGER NOT NULL,
    from_account_id INTEGER NOT NULL,
    to_account_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    merchant_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    memo VARCHAR(1023) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,        
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_bookkeeper_entries_memo ON bookkeeper_entries(memo);

-- migrate:down
DROP TABLE bookkeeper_entries;
DROP TABLE bookkeeper_transactions;
DROP TABLE bookkeeper_merchants;
DROP TABLE bookkeeper_categories;
DROP TABLE bookkeeper_accounts;
DROP TABLE bookkeeper_ledgers;
