package queue_worker

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/daisy/workers"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
)

type EmailSend struct {
	Smtp     env.Smtp        `toml:"smtp"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func (p *EmailSend) Execute(ctx context.Context, consumer_name string, queue_name string) error {
	worker := workers.NewSendEmailWorker(&p.Smtp)
	if err := p.RabbitMq.Consume(ctx, consumer_name, queue_name, worker); err != nil {
		return err
	}

	return nil
}
