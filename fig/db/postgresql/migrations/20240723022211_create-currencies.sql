-- migrate:up
CREATE TABLE currencies(
    id SERIAL PRIMARY KEY,
    code CHAR(3) NOT NULL,
    "number" CHAR(3) NOT NULL,
    "name" VARCHAR(127) NOT NULL,
    country VARCHAR(127) NOT NULL,
    units INTEGER NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_currencies_code ON currencies("code");
CREATE INDEX idx_currencies_name ON currencies("name");
CREATE INDEX idx_currencies_number ON currencies("number");
CREATE INDEX idx_currencies_country ON currencies(country);

-- migrate:down
DROP TABLE currencies;
