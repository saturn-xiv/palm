-- migrate:up
CREATE TABLE google_users(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sub VARCHAR(127) NOT NULL,
    email VARCHAR(127) NOT NULL,
    email_verified BOOLEAN NOT NULL,
    hosted_domain VARCHAR(63),
    given_name VARCHAR(31),
    gender VARCHAR(16),
    link VARCHAR(127),
    family_name VARCHAR(31),
    name VARCHAR(63),
    picture VARCHAR(127),
    locale VARCHAR(15),
    credentials JSONB NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at  TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_google_users_sub ON google_users(sub);
CREATE INDEX idx_google_users_email ON google_users(email);
CREATE INDEX idx_google_users_name ON google_users(name) WHERE name IS NOT NULL;
CREATE INDEX idx_google_users_given_name ON google_users(given_name) WHERE given_name IS NOT NULL;
CREATE INDEX idx_google_users_family_name ON google_users(family_name) WHERE family_name IS NOT NULL;
CREATE INDEX idx_google_users_gender ON google_users(gender) WHERE gender IS NOT NULL;
CREATE INDEX idx_google_users_locale ON google_users(locale) WHERE gender IS NOT NULL;
CREATE INDEX idx_google_users_hosted_domain ON google_users(hosted_domain) WHERE hosted_domain IS NOT NULL;

-- migrate:down
DROP TABLE google_users;
