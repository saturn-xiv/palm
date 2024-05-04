package send_sms_worker

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"
	amqp "github.com/rabbitmq/amqp091-go"
)

func Launch(config_file string, consumer_name string, queue_name string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	ctx := context.Background()

	slog.Debug(fmt.Sprintf("open consumer rabbitmq://%s@%s:%d/%s", config.RabbitMq.User, config.RabbitMq.Host, config.RabbitMq.Port, config.RabbitMq.VirtualHost))
	con, err := amqp.Dial(config.RabbitMq.Url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	messages, err := ch.ConsumeWithContext(ctx, queue_name, consumer_name, true, false, false, false, nil)
	if err != nil {
		return nil
	}

	sms := config.Twilio.Open()
	for it := range messages {
		slog.Info(fmt.Sprintf("receive message (%s,%s)", it.MessageId, it.ContentType))
		if err = sms.Handle(ctx, it.Body); err != nil {
			return err
		}
	}

	return nil
}
