package env

import (
	_ "github.com/sijms/go-ora/v2"
)

type Oracle struct {
}

func (p *Oracle) Up() string {
	return "UPDATE {{ .name }} SET run_at = CURRENT_TIMESTAMP WHERE VERSION = $1"
}

func (p *Oracle) Down() string {
	return "UPDATE {{ .name }} SET run_at = NULL WHERE VERSION = $1"
}

func (p *Oracle) All() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} ORDER BY version ASC"
}
func (p *Oracle) ByVersion() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE VERSION = $1"
}
func (p *Oracle) Latest() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE RUN_AT IS NOT NULL ORDER BY VERSION DESC LIMIT 1"
}

func (p *Oracle) Insert() string {
	return "INSERT INTO {{ .name }}(version, name, up, down) VALUES($1, $2, $3, $4)"
}

func (p *Oracle) Version() string {
	return "SELECT VERSION()"
}

func (p *Oracle) CreateSchemaMigrationsTable() string {
	return `
CREATE TABLE IF NOT EXISTS {{ .name }}(
	id SERIAL PRIMARY KEY,
	version CHAR(14) NOT NULL,
	name VARCHAR2(255) NOT NULL,
	up TEXT NOT NULL,
	down TEXT NOT NULL,	
	run_at TIMESTAMP WITHOUT TIME ZONE,
	created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_{{ .name }}_name ON {{ .name }}(name);
CREATE UNIQUE INDEX IF NOT EXISTS idx_{{ .name }}_version ON {{ .name }}(version);
`

}

func (p *Oracle) DropSchemaMigrationsTable() string {
	return `DROP TABLE IF EXISTS {{ .name }};`
}

func (p *Oracle) HibernateSequence() (string, string) {
	return "CREATE SEQUENCE IF NOT EXISTS hibernate_sequence MINVALUE 1 MAXVALUE 2147483647 START WITH 1 INCREMENT BY 1 CACHE 128;", "DROP SEQUENCE IF EXISTS hibernate_sequence;"
}
