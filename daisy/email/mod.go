package email

import (
	"log/slog"

	"github.com/wneessen/go-mail"
	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn-xiv/palm/daisy/email/v2"
)

type Config struct {
	Host     string `toml:"host"`
	User     *User  `toml:"user"`
	Password string `toml:"password"`
}

type User struct {
	Name  string `toml:"name"`
	Email string `toml:"email"`
}

func (p *Config) Open() (*mail.Client, error) {

	return mail.NewClient(p.Host, mail.WithSMTPAuth(mail.SMTPAuthAutoDiscover),
		mail.WithUsername(p.User.Email), mail.WithPassword(p.Password))

}

type EmailSendProtobufConsumer struct {
	client *mail.Client
	from   *v2.Task_Address
}

func NewEmailSendProtobufConsumer(client *mail.Client, from *v2.Task_Address) *EmailSendProtobufConsumer {
	return &EmailSendProtobufConsumer{client, from}
}

func (p *EmailSendProtobufConsumer) Name() string {
	return "email-send.protobuf"
}
func (p *EmailSendProtobufConsumer) Execute(id string, content_type string, body []byte) error {
	var task v2.Task
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	slog.Info("send email", "from", p.from.Email, "to", task.To.Email, "subject", task.Subject)
	msg, err := task.Build(p.from)
	if err != nil {
		return err
	}

	return p.client.DialAndSend(msg)
}
