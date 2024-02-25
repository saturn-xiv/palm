package sms

import (
	"github.com/saturn-xiv/palm/petunia/env"
	"github.com/saturn-xiv/palm/petunia/env/queue"
)

type Config struct {
	Twilio   env.Twilio           `toml:"twilio"`
	RabbitMq queue.RabbitMqConfig `toml:"rabbitmq"`
}
