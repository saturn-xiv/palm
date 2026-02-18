package cups

import (
	"context"
	"log/slog"

	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn-xiv/palm/daisy/cups/v2"
)

type CupsProtobufConsumer struct {
}

func NewCupsProtobufConsumer() *CupsProtobufConsumer {
	return &CupsProtobufConsumer{}
}

func (p *CupsProtobufConsumer) Name() string {
	return "cups.protobuf"
}

// https://man7.org/linux/man-pages/man1/lpr.1.html
func (p *CupsProtobufConsumer) Execute(ctx context.Context, id string, content_type string, body []byte) error {
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
