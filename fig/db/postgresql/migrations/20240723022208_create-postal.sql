-- migrate:up
CREATE TABLE postal_recipients(
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(31) NOT NULL,
    country_code VARCHAR(7) NOT NULL,
    phone VARCHAR(15) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE, 
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,  
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_postal_recipients_name ON postal_recipients("name");
CREATE INDEX idx_postal_recipients_country_code ON postal_recipients(country_code);
CREATE INDEX idx_postal_recipients_phone ON postal_recipients(phone);

CREATE TABLE postal_addresses(
    id SERIAL PRIMARY KEY,    
    street VARCHAR(127) NOT NULL,
    city VARCHAR(63) NOT NULL,
    state VARCHAR(63) NOT NULL,
    country VARCHAR(63) NOT NULL,
    zip_code VARCHAR(15) NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_postal_addresses_street ON postal_addresses(street);
CREATE INDEX idx_postal_addresses_city ON postal_addresses(city);
CREATE INDEX idx_postal_addresses_state ON postal_addresses(state);
CREATE INDEX idx_postal_addresses_country ON postal_addresses(country);
CREATE INDEX idx_postal_addresses_zip_code ON postal_addresses(zip_code);

-- migrate:down
DROP TABLE postal_addresses;
DROP TABLE postal_recipients;
