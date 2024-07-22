package env

import (
	_ "dm"
)

type DM8 struct {
}

func (p *DM8) Up() string {
	return "UPDATE {{ .name }} SET run_at = CURRENT_TIMESTAMP WHERE VERSION = :1"
}

func (p *DM8) Down() string {
	return "UPDATE {{ .name }} SET run_at = NULL WHERE VERSION = :1"
}

func (p *DM8) All() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} ORDER BY version ASC"
}
func (p *DM8) ByVersion() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE VERSION = :1"
}
func (p *DM8) Latest() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE RUN_AT IS NOT NULL ORDER BY VERSION DESC LIMIT 1"
}

func (p *DM8) Insert() string {
	return "INSERT INTO {{ .name }}(version, name, up, down) VALUES(:1, :2, :3, :4)"
}

func (p *DM8) Version() string {
	return "SELECT BANNER FROM V$VERSION LIMIT 1"
}

func (p *DM8) CreateSchemaMigrationsTable() string {
	return `
BEGIN
	EXECUTE IMMEDIATE 'CREATE TABLE IF NOT EXISTS {{ .name }}(
		id INT IDENTITY(1, 1) NOT NULL,
		version CHAR(14) NOT NULL,
		name VARCHAR(255) NOT NULL,
		up TEXT NOT NULL,
		down TEXT NOT NULL,
		run_at TIMESTAMP,
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		NOT CLUSTER PRIMARY KEY(id)
	)';
	EXECUTE IMMEDIATE 'CREATE INDEX IF NOT EXISTS idx_{{ .name }}_name ON {{ .name }}(name)';
	EXECUTE IMMEDIATE 'CREATE UNIQUE INDEX IF NOT EXISTS idx_{{ .name }}_version ON {{ .name }}(version)';
END;
`
}

func (p *DM8) DropSchemaMigrationsTable() string {
	return `
BEGIN
	DROP TABLE IF EXISTS {{ .name }};
`
}

func (p *DM8) HibernateSequence() (string, string) {
	return "CREATE SEQUENCE IF NOT EXISTS hibernate_sequence MINVALUE 1 MAXVALUE 2147483647 START WITH 1 INCREMENT BY 1 CACHE 128;", "DROP SEQUENCE IF EXISTS hibernate_sequence;"
}
