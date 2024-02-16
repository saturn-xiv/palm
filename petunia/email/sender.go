package email

import (
	"context"
	"os"
	"time"

	amqp "github.com/rabbitmq/amqp091-go"
	log "github.com/sirupsen/logrus"
	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	pb "github.com/saturn-xiv/palm/petunia/email/v2"
)

type SendEmailConsumer struct {
	from   string
	dialer *gomail.Dialer
}

func NewSendEmailConsumer(from string, dialer *gomail.Dialer) SendEmailConsumer {
	return SendEmailConsumer{
		from:   from,
		dialer: dialer,
	}
}
func (p *SendEmailConsumer) Consume(rabbitmq string, consumer_name string, queue_name string) error {
	con, err := amqp.Dial(rabbitmq)
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
		if err = p.consume(msg.MessageId, msg.ContentType, msg.Body); err != nil {
			return err
		}

	}
	return nil
}

func (p *SendEmailConsumer) consume(id string, content_type string, body []byte) error {
	var task pb.SendEmailTask
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	log.Infof("send email to %s<%s>: (%s)", task.To.Name, task.To.Email, task.Subject)

	msg := gomail.NewMessage()
	msg.SetHeader("From", p.from)
	msg.SetAddressHeader("To", task.To.Email, task.To.Name)

	{
		items := []string{}
		for _, it := range task.Cc {
			log.Debugf("cc %s<%s>", it.Name, it.Email)
			items = append(items, msg.FormatAddress(it.Email, it.Name))
		}
		msg.SetHeader("Cc", items...)
	}
	{
		items := []string{}
		for _, it := range task.Bcc {
			log.Debugf("bcc %s<%s>", it.Name, it.Email)
			items = append(items, msg.FormatAddress(it.Email, it.Name))
		}
		msg.SetHeader("Bcc", items...)
	}

	msg.SetHeader("Subject", task.Subject)
	if task.Body.Html {
		msg.SetBody("text/html", task.Body.Payload)
	} else {
		msg.SetBody("text/plain", task.Body.Payload)
	}

	for _, it := range task.Attachments {
		log.Debugf("attach %s(%s)", it.Title, it.ContentType)
		file, err := os.CreateTemp(id, it.Title)
		if err != nil {
			return err
		}
		defer os.Remove(file.Name())
		if _, err = file.Write(it.Payload); err != nil {
			return err
		}
		if err = file.Close(); err != nil {
			return err
		}
		msg.Attach(file.Name())
	}

	return p.dialer.DialAndSend(msg)
}
