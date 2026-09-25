-- migrate:up
CREATE TABLE lavender_tasks(
    id BIGSERIAL PRIMARY KEY,
    uid VARCHAR(36) NOT NULL,
    email VARCHAR(63) NOT NULL,
    job JSONB NOT NULL,
    script TEXT NOT NULL,
    args JSONB NOT NULL,
    output JSONB NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_lavender_tasks_uid ON lavender_tasks(uid);
CREATE INDEX idx_lavender_tasks_email ON lavender_tasks(email);

-- migrate:down
DROP TABLE lavender_tasks;
