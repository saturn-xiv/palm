package queue_worker

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/daisy/workers"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
)

type SmsSend struct {
	Twilio   env.Twilio      `toml:"twilio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func (p *SmsSend) Execute(ctx context.Context, consumer_name string, queue_name string) error {
	worker := workers.NewSendSmsWorker(&p.Twilio)
	if err := p.RabbitMq.Consume(ctx, consumer_name, queue_name, worker); err != nil {
		return err
	}

	return nil
}
