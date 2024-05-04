package env

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/apache/thrift/lib/go/thrift"
	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"

	v1 "github.com/saturn-xiv/palm/tuberose/services/v1"
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

func (p *SendSmsWorker) Handle(ctx context.Context, message []byte) error {
	var task v1.SmsSendTask
	{
		t := thrift.NewTMemoryBufferLen(1024 * 1024 * (1 << 4))
		defer t.Close()

		p := thrift.NewTBinaryProtocolConf(t, &thrift.TConfiguration{})
		de := &thrift.TDeserializer{
			Transport: t,
			Protocol:  p,
		}
		if err := de.Read(ctx, &task, message); err != nil {
			return err
		}
	}
	for _, to := range task.To {
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Body)
		if task.Callback != nil {
			params.SetStatusCallback(*task.Callback)
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
