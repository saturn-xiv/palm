package workers

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"

	v2 "github.com/saturn-xiv/palm/jasmine/services/sms/v2"
)

type SendSmsWorker struct {
	client *twilio.RestClient
	from   string
}

func NewSendSmsWorker(client *twilio.RestClient, from string) *SendSmsWorker {
	return &SendSmsWorker{client: client, from: from}
}

func (p *SendSmsWorker) Handle(ctx context.Context, message []byte) error {
	var task v2.SmsSendTask

	for _, to := range task.To {
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Message)
		if task.CallbackUrl != nil {
			params.SetStatusCallback(*task.CallbackUrl)
		}
		slog.Info(fmt.Sprintf("send sms(%s) => %s", task.Message, to))
		res, err := p.client.Api.CreateMessage(params)
		if err != nil {
			slog.Error(fmt.Sprintf("%d %s", *res.ErrorCode, *res.ErrorMessage))
			return err
		}
	}
	return nil
}
