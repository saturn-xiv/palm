package email_send_consumer

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	pb "github.com/saturn-xiv/palm/lilac/email/v2"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
)

type Config struct {
	Smtp     env.Smtp        `toml:"smtp"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func Launch(name string, config_file string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	worker := config.Smtp.Open()
	ctx := context.Background()
	return config.RabbitMq.Consume(ctx, name, env.TaskQueueName((*pb.EmailSendRequest)(nil)), worker)
}
