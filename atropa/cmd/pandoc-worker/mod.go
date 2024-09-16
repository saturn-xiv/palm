package pandoc_worker

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/lily/workers"
)

type Config struct {
	Minio    minio.Config    `toml:"minio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func Launch(config_file string, consumer_name string, queue_name string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	ctx := context.Background()

	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

	worker := workers.NewTeXLiveWorker(s3)
	if err := config.RabbitMq.Consume(ctx, consumer_name, queue_name, worker); err != nil {
		return err
	}

	return nil
}
