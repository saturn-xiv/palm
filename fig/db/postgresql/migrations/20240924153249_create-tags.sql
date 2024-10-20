-- migrate:up
CREATE TABLE tags(
    id SERIAL PRIMARY KEY,
    code VARCHAR(63) NOT NULL,    
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_tags_code ON tags(code);


CREATE TABLE tag_resources(
    id SERIAL PRIMARY KEY,
    tag_id INT NOT NULL,
    resource_type VARCHAR(127) NOT NULL,
    resource_id INT,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_tag_resources_item_type_id ON tag_resources(tag_id, resource_type, resource_id) WHERE resource_id IS NOT NULL;
CREATE UNIQUE INDEX idx_tag_resources_item_type ON tag_resources(tag_id, resource_type) WHERE resource_id IS NULL;
CREATE INDEX idx_tag_resources_type ON tag_resources(resource_type);

-- migrate:down
DROP TABLE tag_resources;
DROP TABLE tags;
