package env

import (
	log "github.com/sirupsen/logrus"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Sqlite3 struct {
	File string `toml:"file"`
}

func (p *Sqlite3) Open(config *gorm.Config) (*gorm.DB, error) {
	log.Infof("open file://%s", p.File)
	return gorm.Open(sqlite.Open(p.File), config)
}
