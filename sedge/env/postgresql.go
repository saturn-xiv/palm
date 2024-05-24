package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/lib/pq"
)

type PostgreSql struct {
}

func (p *PostgreSql) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open postgresql", slog.String("dsn", dsn))
	return sql.Open("postgres", dsn)
}
