-- migrate:up
CREATE TABLE attachments(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    public BOOL NOT NULL DEFAULT FALSE,
    bucket VARCHAR(63) NOT NULL,
    object VARCHAR(63) NOT NULL,
    title VARCHAR(127) NOT NULL,
    size BIGINT NOT NULL,
    content_type VARCHAR(63) NOT NULL,
    uploaded_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_attachments_bucket_object ON attachments(bucket, object);
CREATE INDEX idx_attachments_bucket ON attachments(bucket);
CREATE INDEX idx_attachments_object ON attachments(object);
CREATE INDEX idx_attachments_title ON attachments(title);
CREATE INDEX idx_attachments_content_type ON attachments(content_type);


CREATE TABLE attachment_resources(
    id BIGSERIAL PRIMARY KEY,
    attachment_id BIGINT NOT NULL,
    resource_type VARCHAR(127) NOT NULL,
    resource_id BIGINT,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_attachments_resources_item_type_id ON attachment_resources(attachment_id, resource_type, resource_id) WHERE resource_id IS NOT NULL;
CREATE UNIQUE INDEX idx_attachments_resources_item_type ON attachment_resources(attachment_id, resource_type) WHERE resource_id IS NULL;
CREATE INDEX idx_attachments_resources_type ON attachment_resources(resource_type);

-- migrate:down
DROP TABLE attachment_resources;
DROP TABLE attachments;
