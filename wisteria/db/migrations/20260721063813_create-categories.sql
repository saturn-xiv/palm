-- migrate:up
CREATE TABLE categories(
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(31) NOT NULL,
    "left" INTEGER NOT NULL,
    "right" INTEGER NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_categories_code ON categories(code);

-- migrate:down
DROP TABLE categories;
