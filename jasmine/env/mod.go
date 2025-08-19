package env

import (
	"errors"

	"gorm.io/gorm"
)

const (
	PLUGIN_NAME = "jasmine"
)

type Database struct {
	PostgreSql PostgreSql `toml:"postgresql,omitempty"`
	MySql      MySql      `toml:"mysql,omitempty"`
	SqlServer  SqlServer  `toml:"sqlserver,omitempty"`
	Sqlite3    Sqlite3    `toml:"sqlite3,omitempty"`
}

func (p *Database) Open() (*gorm.DB, error) {
	config := gorm.Config{
		Logger:      &gormLogger{},
		PrepareStmt: true,
	}
	if len(p.PostgreSql.DbName) > 0 {
		return p.PostgreSql.Open(&config)
	}
	if len(p.MySql.DbName) > 0 {

		return p.MySql.Open(&config)
	}
	if len(p.SqlServer.DbName) > 0 {
		return p.SqlServer.Open(&config)
	}
	if len(p.Sqlite3.File) > 0 {
		return p.Sqlite3.Open(&config)
	}
	return nil, errors.New("couldn't found database")

}
