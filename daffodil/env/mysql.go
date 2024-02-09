package env

import (
	"fmt"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

type MySql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

// https://github.com/go-sql-driver/mysql#dsn-data-source-name
func (p *MySql) Url() string {
	return fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=UTC",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}

func (p *MySql) Open() gorm.Dialector {
	return mysql.Open(p.Url())
}
