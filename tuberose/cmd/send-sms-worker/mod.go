package send_sms_worker

import "github.com/saturn-xiv/palm/tuberose/env"

type Config struct {
	RabbitMq env.RabbitMq `toml:"rabbitmq"`
	Twilio   env.Twilio   `toml:"twilio"`
}
