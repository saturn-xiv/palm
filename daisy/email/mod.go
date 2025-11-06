package email

import (
	"log/slog"

	"github.com/wneessen/go-mail"
	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn_xiv/palm/daisy/email/v2"
)

type Config struct {
	Host     string `toml:"host"`
	Username string `toml:"username"`
	Password string `toml:"password"`
}

func (p *Config) Open() (*mail.Client, error) {

	return mail.NewClient(p.Host, mail.WithSMTPAuth(mail.SMTPAuthAutoDiscover),
		mail.WithUsername(p.Username), mail.WithPassword(p.Password))

}

type EmailSendProtobufConsumer struct {
	client *mail.Client
}

func NewEmailSendProtobufConsumer(client *mail.Client) *EmailSendProtobufConsumer {
	return &EmailSendProtobufConsumer{client: client}
}

func (p *EmailSendProtobufConsumer) Name() string {
	return "email-send.protobuf"
}
func (p *EmailSendProtobufConsumer) Execute(id string, content_type string, body []byte) error {
	var task v2.Task
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	slog.Info("send email", "from", task.From.Email, "to", task.To.Email, "subject", task.Subject)
	msg, err := task.Build()
	if err != nil {
		return err
	}

	return p.client.DialAndSend(msg)
}
