package sms_send_consumer

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
)

type Config struct {
	Twilio   env.Twilio      `toml:"twilio"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func Launch(name string, queue string, config_file string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	worker := config.Twilio.Open()
	ctx := context.Background()
	return config.RabbitMq.Consume(ctx, name, queue, worker)
}
