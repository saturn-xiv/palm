-- migrate:up
CREATE TABLE daffodil_books(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    "name" VARCHAR(63) NOT NULL,
    description VARCHAR(511) NOT NULL,
    cover VARCHAR(255),    
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_daffodil_books_user_name ON daffodil_books(user_id, "name");
CREATE INDEX idx_daffodil_books_name ON daffodil_books("name");
CREATE TABLE daffodil_accounts(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    "name" VARCHAR(63) NOT NULL,
    currency CHAR(3) NOT NULL,
    "type" INT NOT NULL,
    description VARCHAR(511) NOT NULL,
    cover VARCHAR(255),    
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_daffodil_accounts_user_book_name ON daffodil_accounts(user_id, book_id, "name");
CREATE INDEX idx_daffodil_accounts_name ON daffodil_accounts("name");
CREATE INDEX idx_daffodil_accounts_currency ON daffodil_accounts(currency);
CREATE INDEX idx_daffodil_accounts_type ON daffodil_accounts("type");
CREATE INDEX idx_daffodil_accounts_description ON daffodil_accounts(description);

CREATE TABLE daffodil_merchants(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    "name" VARCHAR(63) NOT NULL,
    address VARCHAR(255),
    contact VARCHAR(255),
    description VARCHAR(511),
    cover VARCHAR(255),    
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    "version" INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX daffodil_merchants_user_book_name ON daffodil_merchants(user_id, book_id, "name");
CREATE INDEX daffodil_merchants_name ON daffodil_merchants("name");
CREATE INDEX daffodil_merchants_address ON daffodil_merchants(address) WHERE address IS NOT NULL;
CREATE INDEX daffodil_merchants_contact ON daffodil_merchants(contact) WHERE contact IS NOT NULL;
CREATE INDEX daffodil_merchants_description ON daffodil_merchants(description) WHERE description IS NOT NULL;

CREATE TABLE daffodil_transactions(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    source_account_id BIGINT NOT NULL,
    destination_account_id BIGINT NOT NULL,
    merchant_id BIGINT NOT NULL,
    "type" BIGINT NOT NULL,       
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    summary VARCHAR(511) NOT NULL,     
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX daffodil_transactions_currency ON daffodil_transactions(currency);
CREATE INDEX daffodil_transactions_summary ON daffodil_transactions(summary);

CREATE TABLE daffodil_transaction_trash(
    id BIGSERIAL PRIMARY KEY,
    transaction_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    book_id BIGINT NOT NULL,
    source_account_id BIGINT NOT NULL,
    destination_account_id BIGINT NOT NULL,
    merchant_id BIGINT NOT NULL,
    "type" BIGINT NOT NULL,
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    summary VARCHAR(511) NOT NULL,      
    deleted_reason VARCHAR(255) NOT NULL,
    deleted_by BIGINT NOT NULL,    
    deleted_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX daffodil_transaction_trash_currency ON daffodil_transaction_trash(currency);
CREATE INDEX daffodil_transaction_trash_summary ON daffodil_transaction_trash(summary);
CREATE INDEX daffodil_transaction_trash_deleted_reason ON daffodil_transaction_trash(deleted_reason);

CREATE TABLE daffodil_transaction_attachments(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,    
    transaction_id BIGINT NOT NULL,    
    bucket VARCHAR(63) NOT NULL,
    "object" VARCHAR(63) NOT NULL,
    title VARCHAR(63) NOT NULL,
    content_type VARCHAR(63) NOT NULL,
    published_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX daffodil_transaction_attachments_bucket_object ON daffodil_transaction_attachments(bucket, "object");
CREATE INDEX daffodil_transaction_attachments_bucket ON daffodil_transaction_attachments(bucket);
CREATE INDEX daffodil_transaction_attachments_object ON daffodil_transaction_attachments("object");
CREATE INDEX daffodil_transaction_attachments_title ON daffodil_transaction_attachments(title);
CREATE INDEX daffodil_transaction_attachments_content_type ON daffodil_transaction_attachments(content_type);

-- migrate:down
DROP TABLE daffodil_transaction_trash;
DROP TABLE daffodil_transaction_attachments;
DROP TABLE daffodil_transactions;
DROP TABLE daffodil_merchants;
DROP TABLE daffodil_accounts;
DROP TABLE daffodil_books;
