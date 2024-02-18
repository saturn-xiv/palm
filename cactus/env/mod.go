package env

type Config struct {
	Twilio   Twilio   `toml:"twilio"`
	RabbitMq RabbitMq `toml:"rabbitmq"`
}
