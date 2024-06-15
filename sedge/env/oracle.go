package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/sijms/go-ora/v2"
)

type Oracle struct {
}

func (p *Oracle) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open oracle", slog.String("dsn", dsn))
	return sql.Open("oracle", dsn)
}
