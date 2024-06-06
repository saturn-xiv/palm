package env

import (
	"crypto/tls"

	"gopkg.in/gomail.v2"

	"github.com/saturn-xiv/palm/atropa/workers"
)

type Smtp struct {
	Host     string   `toml:"host"`
	Port     int      `toml:"port"`
	User     string   `toml:"user"`
	Password string   `toml:"password"`
	Cc       []string `toml:"cc"`
	Bcc      []string `toml:"bcc"`
}

func (p *Smtp) Open() *workers.SendEmailWorker {
	dialer := gomail.NewDialer(p.Host, p.Port, p.User, p.Password)
	dialer.TLSConfig = &tls.Config{InsecureSkipVerify: true}
	return workers.NewSendEmailWorker(dialer, p.User, p.Cc, p.Bcc)
}
