package env

type Config struct {
	Redis      Redis      `toml:"redis"`
	PostgreSql PostgreSql `toml:"postgresql"`
}
