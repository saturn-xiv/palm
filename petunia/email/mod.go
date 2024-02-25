package email

import (
	env_email "github.com/saturn-xiv/palm/petunia/env/email"
	"github.com/saturn-xiv/palm/petunia/env/rabbitmq"
)

type Config struct {
	Smtp     env_email.Smtp  `toml:"redis"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}
