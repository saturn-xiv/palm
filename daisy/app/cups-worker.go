package app

import (
	"context"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/cups"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type CupsWorkerConfig struct {
	RabbitMQ *queue.RabbitMQ `toml:"rabbitmq"`
}

func LaunchCupsWorker(config_file string, queue string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config CupsWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	consumer := cups.NewCupsProtobufConsumer()
	ctx := context.Background()
	return config.RabbitMQ.Consume(ctx, queue, consumer)
}
