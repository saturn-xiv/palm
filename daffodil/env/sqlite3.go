package env

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Sqlite3 struct {
	File string `toml:"file"`
}

func (p *Sqlite3) Open() gorm.Dialector {
	return sqlite.Open(p.File)
}
