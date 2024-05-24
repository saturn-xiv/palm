package env

import "fmt"

type Oracle struct {
	Host       string `toml:"host"`
	Port       uint16 `toml:"port"`
	TableSpace string `toml:"tablespace"`
	User       string `toml:"user"`
	Password   string `toml:"password"`
}

func (p *Oracle) Url() string {
	return fmt.Sprintf("oracle://%s:%s@%s:%d?database=%s",
		p.User, p.Password, p.Host, p.Port, p.TableSpace,
	)
}
