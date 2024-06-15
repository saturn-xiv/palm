package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/go-sql-driver/mysql"
)

type MySql struct {
}

func (p *MySql) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open mysql", slog.String("dsn", dsn))
	return sql.Open("mysql", dsn)
}
