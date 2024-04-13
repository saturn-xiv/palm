package rabbitmq

import (
	"context"
	"fmt"

	"github.com/google/uuid"
	amqp "github.com/rabbitmq/amqp091-go"
	log "github.com/sirupsen/logrus"
)

type Consumer interface {
	Handle(id string, content_type string, body []byte) error
}

// type HandlerFunc func(id string, content_type string, body []byte) error

type Config struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	VirtualHost string `toml:"virtual-host"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
}

func (p *Config) Produce(ctx context.Context, exchange string, routing_key string, content_type string, message []byte) error {
	log.Debugf("open rabbitmq://%s@%s:%d/%s", p.User, p.Host, p.Port, p.VirtualHost)
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

	message_id := uuid.New().String()
	log.Infof("publish message(%s) to (%s,%s)", message_id, exchange, routing_key)
	return ch.PublishWithContext(ctx, exchange, routing_key, false, false, amqp.Publishing{
		MessageId:   message_id,
		ContentType: content_type,
		Body:        message,
	})

}

func (p *Config) Consume(ctx context.Context, name string, queue string, consumer Consumer) error {
	log.Debugf("open rabbitmq://%s@%s:%d/%s", p.User, p.Host, p.Port, p.VirtualHost)
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
		log.Infof("receive message (%s,%s)", it.MessageId, it.ContentType)
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
