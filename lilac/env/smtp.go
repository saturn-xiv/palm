package env

import (
	"crypto/tls"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"google.golang.org/protobuf/proto"
	"gopkg.in/gomail.v2"

	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type Smtp struct {
	Host     string   `toml:"host"`
	Port     int      `toml:"port"`
	User     string   `toml:"user"`
	Password string   `toml:"password"`
	Cc       []string `toml:"cc"`
	Bcc      []string `toml:"bcc"`
}

func (p *Smtp) Open() *SendEmailWorker {
	dailer := gomail.NewDialer(p.Host, p.Port, p.User, p.Password)
	dailer.TLSConfig = &tls.Config{InsecureSkipVerify: true}
	return &SendEmailWorker{
		dailer: dailer,
		from:   p.User,
	}
}

type SendEmailWorker struct {
	dailer *gomail.Dialer
	from   string
	cc     []string
	bcc    []string
}

func (p *SendEmailWorker) Handle(_id string, _content_type string, body []byte) error {
	var task pb.EmailSendRequest
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	slog.Info(fmt.Sprintf("send email(%s) => %s", task.Subject, task.To.Display()))

	msg := gomail.NewMessage()
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
		msg.SetBody("text/plain", task.Body.Payload)
	} else {
		msg.SetBody("text/html", task.Body.Payload)
	}

	{
		dir, err := os.MkdirTemp("", "emails-")
		if err != nil {
			return err
		}
		defer os.RemoveAll(dir)
		for _, it := range task.Attachments {
			file := filepath.Join(dir, it.Title)
			if err := os.WriteFile(file, it.Payload, 0600); err != nil {
				return err
			}
			if it.Inline {
				msg.Embed(file)
			} else {
				msg.Attach(file)
			}

		}
	}
	return nil
}
