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
