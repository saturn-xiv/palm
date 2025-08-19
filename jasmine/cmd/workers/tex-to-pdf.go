package workers

import (
	"context"
	"log/slog"

	"github.com/BurntSushi/toml"
	"github.com/minio/minio-go/v7"
	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/rabbitmq"
	v2 "github.com/saturn-xiv/palm/jasmine/services/tex/v2"
)

func LaunchTexToPdfConsumer(config_file string, queue string) error {
	slog.Info("start sms-send consumer", slog.String("queue", queue))
	slog.Debug("load configuration", slog.String("file", config_file))
	ctx := context.Background()
	var config texToPdfWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}
	consumer := newTexToPdfWorker(s3)
	return config.RabbitMQ.Consume(ctx, queue, consumer)
}

type texToPdfWorkerConfig struct {
	Minio    env.Minio       `toml:"minio"`
	RabbitMQ rabbitmq.Config `toml:"rabbitmq"`
}
type texToPdfWorker struct {
	s3 *minio.Client
}

func newTexToPdfWorker(s3 *minio.Client) *texToPdfWorker {
	return &texToPdfWorker{s3: s3}
}

func (p *texToPdfWorker) Handle(id string, content_type string, message []byte) error {
	var task v2.TexToPdfTask
	if err := proto.Unmarshal(message, &task); err != nil {
		return err
	}
	// TODO
	return nil
}
