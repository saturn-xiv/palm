package env

import "fmt"

type RabbitMq struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	VirtualHost string `toml:"virtual-host"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
}

func (p *RabbitMq) Url() string {
	return fmt.Sprintf("amqp://%s:%s@%s:%d/%s",
		p.User, p.Password, p.Host, p.Port, p.VirtualHost,
	)
}
