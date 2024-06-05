package workers

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"

	v2 "github.com/saturn-xiv/palm/atropa/services/v2"
)

type SendSmsWorker struct {
	client *twilio.RestClient
	from   string
}

func NewSendSmsWorker(client *twilio.RestClient, from string) *SendSmsWorker {
	return &SendSmsWorker{client: client, from: from}
}

func (p *SendSmsWorker) Handle(ctx context.Context, message []byte) error {
	var task v2.SmsSendRequest

	for _, to := range task.To {
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Body)
		if task.CallbackUri != nil {
			params.SetStatusCallback(*task.CallbackUri)
		}
		slog.Info(fmt.Sprintf("send sms(%s) => %s", task.Body, to))
		res, err := p.client.Api.CreateMessage(params)
		if err != nil {
			slog.Error(fmt.Sprintf("%d %s", *res.ErrorCode, *res.ErrorMessage))
			return err
		}
	}
	return nil
}
