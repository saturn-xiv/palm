package env

import (
	"database/sql"
	"log/slog"

	_ "dm"
)

// dm://userName:password@ip:port
type DM8 struct {
}

func (p *DM8) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open dm8", slog.String("dsn", dsn))
	return sql.Open("dm", dsn)
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

func (p *DM8) CreateTable() string {
	return `
CREATE TABLE IF NOT EXISTS {{ .name }}(
	id INT IDENTITY(1, 1) NOT NULL,
	version CHAR(14) NOT NULL,
	name VARCHAR(63) NOT NULL,
	up TEXT NOT NULL,
	down TEXT NOT NULL,	
	run_at TIMESTAMP,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	NOT CLUSTER PRIMARY KEY(id)
);
CREATE INDEX IF NOT EXISTS idx_{{ .name }}_name ON {{ .name }}(name);
CREATE UNIQUE INDEX IF NOT EXISTS idx_{{ .name }}_version ON {{ .name }}(version);
`

}

func (p *DM8) Create() string {
	return `
CREATE TABLESPACE "demo" DATAFILE '/var/lib/dm8/demo.dbf' SIZE 10240;
ALTER TABLESPACE "demo" DATAFILE '/var/lib/dm8/demo.dbf' AUTOEXTEND ON NEXT 512 MAXSIZE 102400;

CREATE USER "www" IDENTIFIED BY "change-ME@2024" HASH WITH SHA512 SALT ENCRYPT BY "123456" DEFAULT TABLESPACE "demo" DEFAULT INDEX TABLESPACE "demo";

GRANT "DBA" TO "www";

`
}
func (p *DM8) Drop() string {
	return `
DROP TABLESPACE "demo";
`
}
