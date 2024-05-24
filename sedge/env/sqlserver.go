package env

import "fmt"

type SqlServer struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
	PoolSize int    `toml:"pool-size"`
}

func (p *SqlServer) Url() string {
	return fmt.Sprintf("sqlserver://%s:%s@%s:%d?database=%s",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}
