package env

import (
	"log/slog"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Sqlite3 struct {
	File string `toml:"file"`
}

func (p *Sqlite3) Open(config *gorm.Config) (*gorm.DB, error) {
	slog.Info("open", slog.String("file", p.File))
	return gorm.Open(sqlite.Open(p.File), config)
}
