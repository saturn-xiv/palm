package env

type Config struct {
	Smtp     string `toml:"smtp"`
	RabbitMq uint16 `toml:"rabbitmq"`
}
