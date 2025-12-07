package app

import (
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/email"
	v2 "github.com/saturn-xiv/palm/daisy/email/v2"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type EmailSendWorkerConfig struct {
	RabbitMQ *queue.RabbitMQ `toml:"rabbitmq"`
	Smtp     *email.Config   `toml:"smtp"`
}

func LaunchEmailSendWorker(config_file string, queue string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config EmailSendWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	smtp, err := config.Smtp.Open()
	if err != nil {
		return err
	}
	consumer := email.NewEmailSendProtobufConsumer(smtp, &v2.Task_Address{Name: config.Smtp.User.Name, Email: config.Smtp.User.Email})
	return config.RabbitMQ.Consume(queue, consumer)
}
