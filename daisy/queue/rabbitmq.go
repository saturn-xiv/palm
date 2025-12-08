package queue

import (
	"context"
	"fmt"
	"log/slog"
	"os"

	"github.com/google/uuid"
	amqp "github.com/rabbitmq/amqp091-go"
	"google.golang.org/protobuf/proto"
)

var (
	// https://developers.cloudflare.com/speed/optimization/content/compression/
	ContentType_ApplicationXProtobuf = "application/x-protobuf"
)

type RabbitMQ struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
	VirtualHost string `toml:"virtual-host"`
}

func (p *RabbitMQ) Publish(ctx context.Context, exchange string, content_type string, body []byte) error {
	con, err := amqp.Dial(p.url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	if err = ch.ExchangeDeclare(exchange, "fanout", true, false, false, false, nil); err != nil {
		return err
	}

	return send(ctx, ch, exchange, "", content_type, body)
}

func (p *RabbitMQ) Subscribe(ctx context.Context, exchange string, consumer Consumer) error {
	con, err := amqp.Dial(p.url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	queue, err := ch.QueueDeclare("", false, true, true, false, nil)
	if err != nil {
		return err
	}
	if err = ch.QueueBind(queue.Name, "", exchange, false, nil); err != nil {
		return err
	}
	for {
		if err = receive(ctx, ch, queue.Name, consumer); err != nil {
			return err
		}
	}
}

func (p *RabbitMQ) ProduceProtobuf(ctx context.Context, queue string, task proto.Message) error {
	body, err := proto.Marshal(task)
	if err != nil {
		return err
	}
	return p.Produce(ctx, queue, ContentType_ApplicationXProtobuf, body)
}
func (p *RabbitMQ) Produce(ctx context.Context, queue string, content_type string, body []byte) error {
	con, err := amqp.Dial(p.url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	if _, err = ch.QueueDeclare(queue, true, false, false, false, nil); err != nil {
		return err
	}
	return send(ctx, ch, "", queue, content_type, body)
}

func (p *RabbitMQ) Consume(ctx context.Context, queue string, consumer Consumer) error {
	con, err := amqp.Dial(p.url())
	if err != nil {
		return err
	}
	defer con.Close()
	ch, err := con.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()

	for {
		if err := receive(ctx, ch, queue, consumer); err != nil {
			return err
		}
	}
}

func (p *RabbitMQ) url() string {
	return fmt.Sprintf("amqp://%s:%s@%s:%d/%s", p.User, p.Password, p.Host, p.Port, p.VirtualHost)
}

func send(ctx context.Context, channel *amqp.Channel, exchange string, routing_key string, content_type string, body []byte) error {
	id := uuid.New().String()
	slog.Info("send queue message", "id", id, "exchange", exchange, "routing-key", routing_key, "content-type", content_type)

	return channel.PublishWithContext(ctx,
		exchange,
		routing_key,
		false,
		false,
		amqp.Publishing{
			MessageId:   id,
			ContentType: content_type,
			Body:        body,
		})

}

func receive(ctx context.Context, channel *amqp.Channel, queue string, consumer Consumer) error {
	host, err := os.Hostname()
	if err != nil {
		return err
	}
	name := fmt.Sprintf("%s@%s.%d", consumer.Name(), host, os.Getpid())
	slog.Info("start consumer", "queue", queue, "name", name)
	msgs, err := channel.Consume(queue, name, true, false, false, false, nil)
	if err != nil {
		return err
	}
	for it := range msgs {
		slog.Info("received message", "id", it.MessageId, "content-type", it.ContentType)
		if err = consumer.Execute(ctx, it.MessageId, it.ContentType, it.Body); err != nil {
			return err
		}
	}
	return nil
}
