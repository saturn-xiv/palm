package env

import (
	"fmt"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	pb "github.com/saturn-xiv/palm/lilac/sms/v2"
)

type Twilio struct {
	AccountSid string `toml:"account-sid"`
	AuthToken  string `toml:"auth-token"`
	From       string `toml:"from"`
}

func (p *Twilio) Open() *SendSmsWorker {
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: p.AccountSid,
		Password: p.AuthToken,
	})
	return &SendSmsWorker{
		client: client, from: p.From,
	}
}

type SendSmsWorker struct {
	client *twilio.RestClient
	from   string
}

func (p *SendSmsWorker) Handle(_id string, _content_type string, body []byte) error {
	var task pb.SmsSendRequest
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	for _, to := range task.To {
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Message)
		if task.Callback != nil {
			params.SetStatusCallback(*task.Callback)
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
