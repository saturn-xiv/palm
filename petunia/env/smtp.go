package env

import (
	"crypto/tls"

	"gopkg.in/gomail.v2"
)

type Address struct {
	Name  string `toml:"name"`
	Email string `toml:"email"`
}

type Smtp struct {
	Host     string  `toml:"host"`
	Port     uint16  `toml:"port"`
	User     Address `toml:"user"`
	Password string  `toml:"password"`
}

func (p *Smtp) From() string {
	msg := gomail.NewMessage()
	return msg.FormatAddress(p.User.Email, p.User.Name)
}

func (p *Smtp) Open() *gomail.Dialer {
	cli := gomail.NewDialer(p.Host, int(p.Port), p.User.Email, p.Password)
	cli.TLSConfig = &tls.Config{InsecureSkipVerify: true}
	return cli
}
