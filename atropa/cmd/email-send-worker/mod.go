package email_send_worker

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/atropa/daisy/workers"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
)

type Config struct {
	Smtp     env.Smtp        `toml:"smtp"`
	RabbitMq rabbitmq.Config `toml:"rabbitmq"`
}

func Launch(config_file string, consumer_name string, queue_name string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	ctx := context.Background()

	worker := workers.NewSendEmailWorker(&config.Smtp)
	if err := config.RabbitMq.Consume(ctx, consumer_name, queue_name, worker); err != nil {
		return err
	}

	return nil
}
