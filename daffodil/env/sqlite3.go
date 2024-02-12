package env

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Sqlite3 struct {
	File string `toml:"file"`
}

func (p *Sqlite3) Open() (*gorm.DB, error) {
	return gorm.Open(sqlite.Open(p.File), &gorm.Config{})
}
