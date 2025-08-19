package env

import (
	"crypto/tls"

	"gopkg.in/gomail.v2"
)

type Smtp struct {
	Host     string   `toml:"host"`
	Port     int      `toml:"port"`
	User     string   `toml:"user"`
	Password string   `toml:"password"`
	Cc       []string `toml:"cc"`
	Bcc      []string `toml:"bcc"`
}

func (p *Smtp) Open() *gomail.Dialer {
	dialer := gomail.NewDialer(p.Host, p.Port, p.User, p.Password)
	dialer.TLSConfig = &tls.Config{InsecureSkipVerify: true}
	return dialer
}
