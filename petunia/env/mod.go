package env

type Config struct {
	Smtp     Smtp     `toml:"smtp"`
	RabbitMq RabbitMq `toml:"rabbitmq"`
}
