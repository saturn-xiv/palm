-- migrate:up
CREATE TABLE categories(
    id SERIAL PRIMARY KEY,
    code VARCHAR(63) NOT NULL,
    "left" INT NOT NULL,
    "right" INT NOT NULL,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_categories_code ON categories(code);
INSERT INTO categories(code, "left", "right", updated_at) VALUES('root', 1, 1, CURRENT_TIMESTAMP);

CREATE TABLE category_resources(
    id SERIAL PRIMARY KEY,
    category_id INT NOT NULL,
    resource_type VARCHAR(127) NOT NULL,
    resource_id INT,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_category_resources_item_type_id ON category_resources(category_id, resource_type, resource_id) WHERE resource_id IS NOT NULL;
CREATE UNIQUE INDEX idx_category_resources_item_type ON category_resources(category_id, resource_type) WHERE resource_id IS NULL;
CREATE INDEX idx_category_resources_type ON category_resources(resource_type);

-- migrate:down
DROP TABLE category_resources;
DROP TABLE categories;
