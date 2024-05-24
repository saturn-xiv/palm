package env

import "fmt"

type DaMeng8 struct {
	Host     string `toml:"host"`
	Port     uint16 `toml:"port"`
	DbName   string `toml:"dbname"`
	User     string `toml:"user"`
	Password string `toml:"password"`
}

func (p *DaMeng8) Url() string {
	return fmt.Sprintf("dm://%s:%s@%s:%d?database=%s",
		p.User, p.Password, p.Host, p.Port, p.DbName,
	)
}
