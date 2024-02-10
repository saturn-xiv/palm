package tasks

import (
	"os"

	log "github.com/sirupsen/logrus"
	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	"github.com/saturn-xiv/palm/env"
	"github.com/saturn-xiv/palm/env/email"
	gourd_pb "github.com/saturn-xiv/palm/gourd/v2"
)

type SmtpConsumerConfig struct {
	RabbitMq env.RabbitMq `toml:"rabbitmq"`
	Smtp     email.Smtp   `toml:"smtp"`
}

func (p *SmtpConsumerConfig) Open() SmtpConsumerHandler {
	msg := gomail.NewMessage()
	return SmtpConsumerHandler{
		from:   msg.FormatAddress(p.Smtp.User.Email, p.Smtp.User.Name),
		dialer: p.Smtp.Open(),
	}
}

type SmtpConsumerHandler struct {
	from   string
	dialer *gomail.Dialer
}

func (p *SmtpConsumerHandler) Handle(id string, content_type string, body []byte) error {
	task := gourd_pb.SendEmail{}
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
