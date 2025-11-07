package app

import (
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/email"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type EmailSendWorkerConfig struct {
	RabbitMQ *queue.RabbitMQ `toml:"rabbitmq"`
	Smtp     *email.Config   `toml:"smtp"`
}

func LaunchEmailSendWorker(config_file string, queue string, debug bool) error {
	init_logger(debug)

	slog.Debug("load configuration from", "file", config_file)
	var config EmailSendWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	smtp, err := config.Smtp.Open()
	if err != nil {
		return err
	}
	consumer := email.NewEmailSendProtobufConsumer(smtp)
	return config.RabbitMQ.Consume(queue, consumer)
}
