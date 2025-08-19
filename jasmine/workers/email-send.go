package workers

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	v2 "github.com/saturn-xiv/palm/jasmine/services/mail/v2"
)

type SendEmailWorker struct {
	dialer *gomail.Dialer
	from   string
	cc     []string
	bcc    []string
}

func NewSendEmailWorker(dialer *gomail.Dialer, from string, cc []string, bcc []string) *SendEmailWorker {
	return &SendEmailWorker{dialer: dialer, from: from, cc: cc, bcc: bcc}
}

func (p *SendEmailWorker) Handle(ctx context.Context, message []byte) error {
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
