package sms

import (
	"github.com/saturn-xiv/palm/petunia/env"
	"github.com/saturn-xiv/palm/petunia/env/rabbitmq"
)

type Config struct {
	Twilio   env.Twilio      `toml:"twilio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}
