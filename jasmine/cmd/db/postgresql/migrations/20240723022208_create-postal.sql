-- migrate:up
CREATE TABLE postal_recipients(
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(63) NOT NULL,    
    phone VARCHAR(31),
    fax VARCHAR(31),
    email VARCHAR(127),
    whatsapp VARCHAR(63),
    wechat VARCHAR(63),
    deleted_at TIMESTAMP WITHOUT TIME ZONE, 
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,  
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_postal_recipients_name ON postal_recipients("name");
CREATE INDEX idx_postal_recipients_phone ON postal_recipients(phone) WHERE phone IS NOT NULL;
CREATE INDEX idx_postal_recipients_fax ON postal_recipients(fax) WHERE fax IS NOT NULL;
CREATE INDEX idx_postal_recipients_email ON postal_recipients(email) WHERE email IS NOT NULL;
CREATE INDEX idx_postal_recipients_whatsapp ON postal_recipients(whatsapp) WHERE whatsapp IS NOT NULL;
CREATE INDEX idx_postal_recipients_wechat ON postal_recipients(wechat) WHERE wechat IS NOT NULL;

CREATE TABLE postal_addresses(
    id SERIAL PRIMARY KEY,
    unit VARCHAR(7),
    building VARCHAR(31),
    street VARCHAR(127) NOT NULL,
    city VARCHAR(63) NOT NULL,
    province VARCHAR(63) NOT NULL,
    country VARCHAR(63) NOT NULL,    
    zip_code VARCHAR(15) NOT NULL,
    passcode VARCHAR(15),
    google_map VARCHAR(255),
    a_map  VARCHAR(255),
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_postal_addresses_unit ON postal_addresses(unit) WHERE unit IS NOT NULL;
CREATE INDEX idx_postal_addresses_building ON postal_addresses(building) WHERE building IS NOT NULL;
CREATE INDEX idx_postal_addresses_street ON postal_addresses(street);
CREATE INDEX idx_postal_addresses_city ON postal_addresses(city);
CREATE INDEX idx_postal_addresses_province ON postal_addresses(province);
CREATE INDEX idx_postal_addresses_country ON postal_addresses(country);
CREATE INDEX idx_postal_addresses_zip_code ON postal_addresses(zip_code);

-- migrate:down
DROP TABLE postal_addresses;
DROP TABLE postal_recipients;
