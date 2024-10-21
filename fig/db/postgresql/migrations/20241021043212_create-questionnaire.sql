-- migrate:up
CREATE TABLE questionnaire_forms(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    uid VARCHAR(36) NOT NULL,    
    title VARCHAR(255) NOT NULL,    
    description TEXT NOT NULL,
    description_editor VARCHAR(15) NOT NULL,    
    profile BYTEA NOT NULL,
    status VARCHAR(15) NOT NULL,
    locked_at TIMESTAMP WITHOUT TIME ZONE,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_questionnaire_forms_uid ON questionnaire_forms(uid);
CREATE INDEX idx_questionnaire_forms_title ON questionnaire_forms(title);
CREATE INDEX idx_questionnaire_forms_status ON questionnaire_forms(status);

CREATE TABLE questionnaire_fields(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    form_id INTEGER NOT NULL,
    uid VARCHAR(36) NOT NULL,
    label VARCHAR(255) NOT NULL,    
    summary TEXT NOT NULL,    
    profile BYTEA NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_questionnaire_fields_uid ON questionnaire_fields(uid);
CREATE INDEX idx_questionnaire_fields_label ON questionnaire_fields(label);

CREATE TABLE questionnaire_polls(
    id SERIAL PRIMARY KEY,
    form_id INTEGER NOT NULL,
    batch_no VARCHAR(36) NOT NULL,
    field_id INTEGER NOT NULL,
    value BYTEA NOT NULL,
    deleted_at TIMESTAMP WITHOUT TIME ZONE,
    version INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,    
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_questionnaire_batch_and_field ON questionnaire_polls(batch_no, field_id);
CREATE INDEX idx_questionnaire_polls_batch_no ON questionnaire_polls(batch_no);

-- migrate:down
DROP TABLE questionnaire_polls;
DROP TABLE questionnaire_fields;
DROP TABLE questionnaire_forms;
