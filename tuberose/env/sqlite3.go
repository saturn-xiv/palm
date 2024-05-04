package env

import (
	"fmt"
	"log/slog"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Sqlite3 struct {
	File string `toml:"file"`
}

func (p *Sqlite3) Open(config *gorm.Config) (*gorm.DB, error) {
	slog.Info(fmt.Sprintf("open file://%s", p.File))
	return gorm.Open(sqlite.Open(p.File), config)
}
