package workers

import (
	"crypto/tls"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	v2 "github.com/saturn-xiv/palm/atropa/daisy/services/v2"
	"github.com/saturn-xiv/palm/atropa/env"
)

type SendEmailWorker struct {
	dialer *gomail.Dialer
	from   string
	cc     []string
	bcc    []string
}

func NewSendEmailWorker(config *env.Smtp) *SendEmailWorker {
	dialer := gomail.NewDialer(config.Host, config.Port, config.User, config.Password)
	dialer.TLSConfig = &tls.Config{InsecureSkipVerify: true}
	return &SendEmailWorker{dialer: dialer, from: config.User, cc: config.Cc, bcc: config.Bcc}
}

func (p *SendEmailWorker) Handle(id string, content_type string, body []byte) error {
	var task v2.EmailTask

	if content_type == env.APPLICATION_GRPC_PROTO {
		if err := proto.Unmarshal(body, &task); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("unsupported content-type(%s)", content_type)
	}

	slog.Info(fmt.Sprintf("send email(%s) => %s", task.Subject, task.To.Display()))
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
			msg.SetBody("text/plain", task.Body.Text)
		} else {
			msg.SetBody("text/html", task.Body.Text)
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
