package send_email_worker

import "github.com/saturn-xiv/palm/daisy/env"

type Config struct {
	Smtp     env.Smtp     `toml:"smtp"`
	RabbitMq env.RabbitMq `toml:"rabbitmq"`
}
