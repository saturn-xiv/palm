package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/mattn/go-sqlite3"
)

type Sqlite3 struct {
}

func (p *Sqlite3) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open sqlite3", slog.String("dsn", dsn))
	return sql.Open("sqlite3", dsn)
}

func (p *Sqlite3) Up() string {
	return "UPDATE {{ .name }} SET run_at = CURRENT_TIMESTAMP WHERE VERSION = ?"
}

func (p *Sqlite3) Down() string {
	return "UPDATE {{ .name }} SET run_at = NULL WHERE VERSION = ?"
}

func (p *Sqlite3) All() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} ORDER BY version ASC"
}
func (p *Sqlite3) ByVersion() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE VERSION = ?"
}
func (p *Sqlite3) Latest() string {
	return "SELECT version, name, up, down, run_at FROM {{ .name }} WHERE RUN_AT IS NOT NULL ORDER BY VERSION DESC LIMIT 1"
}

func (p *Sqlite3) Insert() string {
	return "INSERT INTO {{ .name }}(version, name, up, down) VALUES(?, ?, ?, ?)"
}

func (p *Sqlite3) Version() string {
	return "SELECT SQLITE_VERSION()"
}

func (p *Sqlite3) CreateSchemaMigrationsTable() string {
	return `
CREATE TABLE IF NOT EXISTS {{ .name }}(
	id INTEGER NOT NULL PRIMARY KEY,
	version CHAR(14) NOT NULL,
	name VARCHAR(255) NOT NULL,
	up TEXT NOT NULL,
	down TEXT NOT NULL,	
	run_at DATETIME,
	created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_{{ .name }}_name ON {{ .name }}(name);
CREATE UNIQUE INDEX IF NOT EXISTS idx_{{ .name }}_version ON {{ .name }}(version);
`

}

func (p *Sqlite3) DropSchemaMigrationsTable() string {
	return `DROP TABLE IF EXISTS {{ .name }};`
}

func (p *Sqlite3) HibernateSequence() (string, string) {
	return "SELECT 1;", "SELECT 0;"
}
