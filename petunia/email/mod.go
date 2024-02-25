package email

import (
	env_email "github.com/saturn-xiv/palm/petunia/env/email"
	env_queue "github.com/saturn-xiv/palm/petunia/env/queue"
)

type Config struct {
	Smtp     env_email.Smtp           `toml:"redis"`
	RabbitMq env_queue.RabbitMqConfig `toml:"rabbitmq"`
}
