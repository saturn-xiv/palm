package queue_worker

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/lily/workers"
)

type Pandoc struct {
	Minio    minio.Config    `toml:"minio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func (p *Pandoc) Execute(ctx context.Context, consumer_name string, queue_name string) error {
	s3, err := p.Minio.Open()
	if err != nil {
		return err
	}

	worker := workers.NewTeXLiveWorker(s3)
	if err := p.RabbitMq.Consume(ctx, consumer_name, queue_name, worker); err != nil {
		return err
	}

	return nil
}
