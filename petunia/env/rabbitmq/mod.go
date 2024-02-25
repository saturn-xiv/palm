package rabbitmq

import (
	"context"
	"fmt"
	"time"

	amqp "github.com/rabbitmq/amqp091-go"
	log "github.com/sirupsen/logrus"

	"github.com/saturn-xiv/palm/petunia/env/queue"
)

type Config struct {
	Host        string `toml:"host"`
	Port        uint16 `toml:"port"`
	User        string `toml:"user"`
	Password    string `toml:"password"`
	VirtualHost string `toml:"virtual-host"`
}

func (p *Config) Open() *Node {
	return &Node{uri: p.URI()}
}

func (p *Config) URI() string {
	return fmt.Sprintf("amqp://%s:%s@%s:%d/%s", p.User, p.Password, p.Host, p.Port, p.VirtualHost)
}

type Node struct {
	uri string
}

func (p *Node) Consume(consumer_name string, queue_name string, consumer_handler queue.Consumer) error {
	con, err := amqp.Dial(p.uri)
	if err != nil {
		return err
	}
	defer con.Close()

	ch, err := con.Channel()
	if err != nil {
		return err
	}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	messages, err := ch.ConsumeWithContext(ctx, queue_name, consumer_name, true, false, false, false, nil)
	if err != nil {
		return err
	}

	for msg := range messages {
		log.Infof("receive message %s@%s", msg.MessageId, msg.ContentType)
		if err = consumer_handler.Handle(msg.MessageId, msg.ContentType, msg.Body); err != nil {
			return err
		}
	}
	return nil
}

func (p *Node) Produce(queue string, message *queue.Message) error {
	return p.send("", queue, message)
}

func (p *Node) Publish(exchange string, message *queue.Message) error {
	return p.send(exchange, "", message)
}

func (p *Node) Bind(exchange_name string, exchange_type string, queue_name string, routing_key string) error {
	con, err := amqp.Dial(p.uri)
	if err != nil {
		return err
	}
	defer con.Close()

	ch, err := con.Channel()
	if err != nil {
		return err
	}
	if err = ch.ExchangeDeclare(exchange_name, exchange_type, true, false, false, false, nil); err != nil {
		return nil
	}

	if _, err = ch.QueueDeclare(queue_name, true, false, false, false, nil); err != nil {
		return err
	}
	if err = ch.QueueBind(queue_name, routing_key, exchange_name, false, nil); err != nil {
		return err
	}
	return nil

}

func (p *Node) send(exchange string, routing_key string, message *queue.Message) error {
	con, err := amqp.Dial(p.uri)
	if err != nil {
		return err
	}
	defer con.Close()

	ch, err := con.Channel()
	if err != nil {
		return err
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	return ch.PublishWithContext(ctx,
		exchange,
		routing_key,
		false,
		false,
		amqp.Publishing{
			ContentType: message.ContentType,
			Body:        message.Body,
			MessageId:   message.Id,
		})

}
