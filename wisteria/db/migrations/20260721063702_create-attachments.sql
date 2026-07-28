-- migrate:up
CREATE TABLE attachments(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    bucket VARCHAR(63) NOT NULL,
    object VARCHAR(63) NOT NULL,
    title VARCHAR(127) NOT NULL,
    size BIGINT NOT NULL,
    content_type VARCHAR(63) NOT NULL,
    public BOOLEAN NOT NULL DEFAULT FALSE,
    uploaded_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_attachments ON attachments(bucket, object);
CREATE INDEX idx_attachments_bucket ON attachments(bucket);
CREATE INDEX idx_attachments_object ON attachments(object);
CREATE INDEX idx_attachments_title ON attachments(title);
CREATE INDEX idx_attachments_content_type ON attachments(content_type);

-- migrate:down
DROP TABLE attachments;
