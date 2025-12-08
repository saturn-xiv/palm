package app

import (
	"context"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/queue"
	"github.com/saturn-xiv/palm/daisy/s3"
	"github.com/saturn-xiv/palm/daisy/tex"
)

type TexWorkerConfig struct {
	RabbitMQ *queue.RabbitMQ `toml:"rabbitmq"`
	Minio    *s3.Config      `toml:"minio"`
}

func TexWorker(config_file string, queue string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config TexWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}
	consumer := tex.NewTexProtobufConsumer(s3)
	ctx := context.Background()
	return config.RabbitMQ.Consume(ctx, queue, consumer)
}
