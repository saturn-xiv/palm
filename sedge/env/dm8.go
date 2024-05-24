package env

import (
	"database/sql"
	"log/slog"

	_ "github.com/saturn-xiv/palm/sedge/vendors/dm8-20240326"
)

// dm://userName:password@ip:port
type DM8 struct {
}

func (p *DM8) Open(dsn string) (*sql.DB, error) {
	slog.Debug("open dm8", slog.String("dsn", dsn))
	return sql.Open("dm", dsn)
}
