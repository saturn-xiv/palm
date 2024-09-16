package env

type Smtp struct {
	Host     string   `toml:"host"`
	Port     int      `toml:"port"`
	User     string   `toml:"user"`
	Password string   `toml:"password"`
	Cc       []string `toml:"cc"`
	Bcc      []string `toml:"bcc"`
}
