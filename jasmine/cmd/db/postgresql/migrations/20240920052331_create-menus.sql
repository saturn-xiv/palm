-- migrate:up
CREATE TABLE menu_items(
    id SERIAL PRIMARY KEY,
    parent_id INT,
    lang VARCHAR(15) NOT NULL,
    location VARCHAR(63) NOT NULL,
    label VARCHAR(63) NOT NULL,
    link VARCHAR(255),
    extra BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INT NOT NULL DEFAULT 0,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_menu_items_lang ON menu_items(lang);
CREATE INDEX idx_menu_items_location ON menu_items(location);
CREATE INDEX idx_menu_items_label ON menu_items(label);


-- migrate:down
DROP TABLE menu_items;
