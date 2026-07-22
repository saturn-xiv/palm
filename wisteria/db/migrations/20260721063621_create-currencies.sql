-- migrate:up
CREATE TABLE currencies(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(127) NOT NULL,
    code VARCHAR(3) NOT NULL,
    country VARCHAR(127) NOT NULL,
    number INTEGER NOT NULL,
    units INTEGER,
    fund BOOLEAN,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_currencies_name ON currencies(name);
CREATE INDEX idx_currencies_code ON currencies(code);
CREATE INDEX idx_currencies_country ON currencies(country);

-- migrate:down
DROP TABLE currencies;
