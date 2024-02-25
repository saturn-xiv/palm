package email

import (
	"github.com/saturn-xiv/palm/petunia/env"
	env_email "github.com/saturn-xiv/palm/petunia/env/email"
)

type Config struct {
	Smtp     env_email.Smtp `toml:"redis"`
	RabbitMq env.RabbitMq   `toml:"rabbitmq"`
}
