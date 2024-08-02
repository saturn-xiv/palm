package workers

import (
	"fmt"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	v2 "github.com/saturn-xiv/palm/atropa/daisy/services/v2"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

type SendSmsWorker struct {
	client *twilio.RestClient
	from   string
}

func NewSendSmsWorker(config *env.Twilio) *SendSmsWorker {
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: config.AccountSid,
		Password: config.AuthToken,
	})
	return &SendSmsWorker{client: client, from: config.From}
}

func (p *SendSmsWorker) Handle(id string, content_type string, body []byte) error {
	var task v2.SmsTask
	if content_type == hibiscus.APPLICATION_GRPC_PROTO {
		if err := proto.Unmarshal(body, &task); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("unsupported content-type(%s)", content_type)
	}

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
