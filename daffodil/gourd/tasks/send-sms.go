package tasks

import (
	"github.com/saturn-xiv/palm/env"
)

type SmsConsumerConfig struct {
	RabbitMq env.RabbitMq `toml:"rabbitmq"`
	Twilio   env.Twilio   `toml:"twilio"`
}

type TwilioSmsConsumerHandler struct{}

func (p *TwilioSmsConsumerHandler) Handle(id string, content_type string, body []byte) error {
	// TODO
	return nil
}
