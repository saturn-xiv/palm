package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/microsoft/go-mssqldb"
)

type SqlServer struct {
}

func (p *SqlServer) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open sqlserver", slog.String("dsn", dsn))
	return sql.Open("sqlserver", dsn)
}
