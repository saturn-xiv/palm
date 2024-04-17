package email_send_consumer

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
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
	return config.RabbitMq.Consume(ctx, name, pb.TaskQueueName((*pb.EmailSendRequest)(nil)), worker)
}
