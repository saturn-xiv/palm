package env

type RabbitMq struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	VirtualHost string `toml:"virtual-host"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
}
