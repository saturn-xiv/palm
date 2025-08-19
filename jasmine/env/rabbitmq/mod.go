package rabbitmq

import (
	"context"
	"fmt"
	"log/slog"
	"reflect"

	"github.com/google/uuid"
	amqp "github.com/rabbitmq/amqp091-go"
	"google.golang.org/protobuf/proto"
)

type Consumer interface {
	Handle(id string, content_type string, body []byte) error
}

type Config struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	VirtualHost string `toml:"virtual-host"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
}

// https://www.rabbitmq.com/tutorials/tutorial-two-go
func (p *Config) ProducePB(ctx context.Context, message proto.Message) error {
	slog.Debug("open producer", slog.String("user", p.User), slog.String("host", p.Host), slog.Int("port", int(p.Port)), slog.String("virtual-host", p.VirtualHost))
	queue := TypeNamePB(message)
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
	if err = p.declare_queue(ch, queue); err != nil {
		return err
	}
	return p.send_protobuf_message(ctx, ch, "", queue, message)
}

// https://www.rabbitmq.com/tutorials/tutorial-three-go
func (p *Config) Publish(ctx context.Context, message proto.Message) error {
	slog.Debug("open publisher", slog.String("user", p.User), slog.String("host", p.Host), slog.Int("port", int(p.Port)), slog.String("virtual-host", p.VirtualHost))
	exchange := TypeNamePB(message)
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

	if err := p.declare_exchange(ch, exchange, "fanout"); err != nil {
		return err
	}
	return p.send_protobuf_message(ctx, ch, exchange, "", message)
}

func (p *Config) declare_exchange(ch *amqp.Channel, name string, kind string) error {
	slog.Debug("declare exchange", slog.String("name", name), slog.String("kind", kind))
	return ch.ExchangeDeclare(name, kind, true, false, false, false, nil)
}

func (p *Config) declare_queue(ch *amqp.Channel, name string) error {
	slog.Debug("declare queue", slog.String("name", name))
	_, err := ch.QueueDeclare(name, true, false, false, false, nil)
	return err
}
func (p *Config) send_protobuf_message(ctx context.Context, ch *amqp.Channel, exchange string, routing_key string, message proto.Message) error {
	buf, err := proto.Marshal(message)
	if err != nil {
		return err
	}
	return p.send(ctx, ch, exchange, routing_key, reflect.TypeOf((*proto.Message)(nil)).String(), buf)
}
func (p *Config) send(ctx context.Context, ch *amqp.Channel, exchange string, routing_key string, content_type string, message []byte) error {
	message_id := uuid.New().String()
	slog.Info("publish message", slog.String("id", message_id), slog.String("exchange", exchange), slog.String("routing-key", routing_key))
	return ch.PublishWithContext(ctx, exchange, routing_key, false, false, amqp.Publishing{
		MessageId:   message_id,
		ContentType: content_type,
		Body:        message,
	})

}

func (p *Config) Consume(ctx context.Context, name string, queue string, consumer Consumer) error {
	slog.Debug("open consumer", slog.String("user", p.User), slog.String("host", p.Host), slog.Int("port", int(p.Port)), slog.String("virtual-host", p.VirtualHost))
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

func (p *Config) Url() string {
	return fmt.Sprintf("amqp://%s:%s@%s:%d/%s",
		p.User, p.Password, p.Host, p.Port, p.VirtualHost,
	)
}

func TypeNamePB(it proto.Message) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
