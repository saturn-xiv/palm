package env

import "fmt"

type MySql struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
	PoolSize int    `toml:"pool-size"`
}

// https://github.com/go-sql-driver/mysql#dsn-data-source-name
func (p *MySql) Url() string {
	return fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=UTC",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}
