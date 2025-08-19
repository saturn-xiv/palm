package workers

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	"github.com/BurntSushi/toml"
	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/rabbitmq"
	v2 "github.com/saturn-xiv/palm/jasmine/services/mail/v2"
)

func LaunchEmailSendConsumer(config_file string, queue string) error {
	slog.Info("start email-send consumer", slog.String("queue", queue))
	slog.Debug("load configuration", slog.String("file", config_file))
	ctx := context.Background()
	var config sendEmailWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	consumer := newSendEmailWorker(config.Smtp.Open(), config.Smtp.User, config.Smtp.Cc, config.Smtp.Bcc)
	return config.RabbitMQ.Consume(ctx, queue, consumer)
}

type sendEmailWorkerConfig struct {
	Smtp     env.Smtp        `toml:"smtp"`
	RabbitMQ rabbitmq.Config `toml:"rabbitmq"`
}

type sendEmailWorker struct {
	dialer *gomail.Dialer
	from   string
	cc     []string
	bcc    []string
}

func newSendEmailWorker(dialer *gomail.Dialer, from string, cc []string, bcc []string) *sendEmailWorker {
	return &sendEmailWorker{dialer: dialer, from: from, cc: cc, bcc: bcc}
}

func (p *sendEmailWorker) Handle(id string, content_type string, message []byte) error {
	var task v2.EmailSendTask
	if err := proto.Unmarshal(message, &task); err != nil {
		return err
	}

	slog.Info("send email", slog.String("subject", task.Subject), slog.String("to", task.To.Display()))
	msg := gomail.NewMessage()
	{
		msg.SetHeader("From", p.from)
		msg.SetHeader("To", msg.FormatAddress(task.To.Email, task.To.Name))
		{
			var cc []string
			cc = append(cc, p.cc...)
			for _, it := range task.Cc {
				cc = append(cc, msg.FormatAddress(it.Email, it.Name))
			}
			msg.SetHeader("Cc", cc...)
		}
		{
			var bcc []string
			bcc = append(bcc, p.bcc...)
			for _, it := range task.Bcc {
				bcc = append(bcc, msg.FormatAddress(it.Email, it.Name))
			}
			msg.SetHeader("Bcc", bcc...)
		}
		msg.SetHeader("Subject", task.Subject)
		if task.Body.Html {
			msg.SetBody("text/plain", task.Body.Content)
		} else {
			msg.SetBody("text/html", task.Body.Content)
		}

		{
			dir, err := os.MkdirTemp("", "emails-")
			if err != nil {
				return err
			}
			defer os.RemoveAll(dir)
			for _, it := range task.Attachments {
				file := filepath.Join(dir, it.Title)
				if err := os.WriteFile(file, it.Body, 0600); err != nil {
					return err
				}
				if it.Inline {
					msg.Embed(file)
				} else {
					msg.Attach(file)
				}
			}
		}
	}

	if err := p.dialer.DialAndSend(msg); err != nil {
		return err
	}
	return nil
}
