package app

import (
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/queue"
	"github.com/saturn-xiv/palm/daisy/sms"
)

type EmailSmsWorkerConfig struct {
	RabbitMQ *queue.RabbitMQ `toml:"rabbitmq"`
	Twilio   *sms.Twilio     `toml:"twilio"`
}

func LaunchSmsSendWorker(config_file string, queue string, debug bool) error {
	init_logger(debug)

	slog.Debug("load configuration from", "file", config_file)
	var config EmailSmsWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	consumer := sms.NewTwilioSmsSendProtobufConsumer(config.Twilio.Open())
	return config.RabbitMQ.Consume(queue, consumer)
}
