-- migrate:up
CREATE TABLE google_oauth2_users(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    subject VARCHAR(127) NOT NULL,    
    email VARCHAR(127),
    email_verified BOOL NOT NULL DEFAULT FALSE,    
    name VARCHAR(63),
    picture VARCHAR(255),
    locale VARCHAR(15),
    code BYTEA NOT NULL,
    token BYTEA NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,    
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_google_oauth2_users_subject ON google_oauth2_users(subject);
CREATE INDEX idx_google_oauth2_users_email ON google_oauth2_users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_google_oauth2_users_name ON google_oauth2_users(name) WHERE name IS NOT NULL;
CREATE INDEX idx_google_oauth2_users_locale ON google_oauth2_users(locale) WHERE locale IS NOT NULL;

-- migrate:down
DROP TABLE google_oauth2_users;
