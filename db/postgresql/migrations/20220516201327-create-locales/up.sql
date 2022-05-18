CREATE TABLE locales (
    id SERIAL PRIMARY KEY,
    lang VARCHAR(16) NOT NULL,
    code VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_locales ON locales(lang, code);
CREATE INDEX idx_locales_lang ON locales(lang);
CREATE INDEX idx_locales_code ON locales(code);
