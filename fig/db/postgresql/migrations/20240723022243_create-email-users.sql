-- migrate:up
CREATE TABLE email_users(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    real_name VARCHAR(63) NOT NULL,    
    nickname VARCHAR(63) NOT NULL,
    email VARCHAR(127) NOT NULL,
    password BYTEA NOT NULL,    
    avatar VARCHAR(255) NOT NULL,    
    confirmed_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_email_users_email ON email_users(email);
CREATE UNIQUE INDEX idx_email_users_nickname ON email_users(nickname);
CREATE INDEX idx_email_users_real_name ON email_users(real_name);

-- migrate:down
DROP TABLE email_users;
