package rabbitmq

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"os/user"

	amqp "github.com/rabbitmq/amqp091-go"
)

type Consumer interface {
	Handle(id string, content_type string, body []byte) error
}

func (p *Config) Consume(ctx context.Context, queue string, consumer Consumer) error {
	name, err := consumer_name()
	if err != nil {
		return err
	}
	slog.Info("start consumer", slog.String("name", name), slog.String("queue", queue))
	slog.Debug("open RabbitMQ", slog.String("user", p.User), slog.String("host", p.Host), slog.Int("port", int(p.Port)), slog.String("virtual-host", p.VirtualHost))
	con, err := amqp.Dial(p.Url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	messages, err := ch.ConsumeWithContext(ctx, queue, name, true, false, false, false, nil)
	if err != nil {
		return nil
	}
	for it := range messages {
		slog.Info("receive message", slog.String("id", it.MessageId), slog.String("content-type", it.ContentType))
		if err = consumer.Handle(it.MessageId, it.ContentType, it.Body); err != nil {
			return err
		}
	}

	return nil
}

func consumer_name() (string, error) {
	cur, err := user.Current()
	if err != nil {
		return "", err
	}
	hn, err := os.Hostname()
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%s.%s.%d", hn, cur.Username, os.Getpid()), nil
}
