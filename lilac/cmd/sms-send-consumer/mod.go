package sms_send_consumer

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/sms/v2"
)

type Config struct {
	Twilio   env.Twilio      `toml:"twilio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func Launch(name string, config_file string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	worker := config.Twilio.Open()
	ctx := context.Background()
	return config.RabbitMq.Consume(ctx, name, env.TaskQueueName((*pb.SmsSendRequest)(nil)), worker)
}
