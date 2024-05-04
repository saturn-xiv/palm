package env

type Config struct {
	RabbitMq string `toml:"rabbitmq"`
	Minio    string `toml:"minio"`
}
