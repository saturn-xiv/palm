package env

import (
	"gorm.io/gorm"
)

type Database struct {
	PostgreSql PostgreSql `toml:"postgresql,omitempty"`
	MySql      MySql      `toml:"mysql,omitempty"`
	SqlServer  SqlServer  `toml:"sqlserver,omitempty"`
}

func (p *Database) Open() (*gorm.DB, error) {
	if len(p.PostgreSql.DbName) > 0 {
		return p.PostgreSql.Open()
	}
	if len(p.MySql.DbName) > 0 {

		return p.MySql.Open()
	}
	if len(p.SqlServer.DbName) > 0 {
		return p.SqlServer.Open()
	}
	it := Sqlite3{File: "tmp/db"}
	return it.Open()
}
