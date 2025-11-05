package sms

import (
	"encoding/json"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"

	v2 "com.github/saturn_xiv/palm/daisy/sms/v2"
)

type Twilio struct {
	AccountSid string
	AuthToken  string
}

func (p *Twilio) Send(task *v2.Task) error {
	client := twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: p.AccountSid,
		Password: p.AuthToken,
	})

	for _, to := range task.To {
		slog.Debug("send sms message", "from", task.From, "to", task.To, "body", task.Body)
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(task.From)
		params.SetBody(task.Body)

		message, err := client.Api.CreateMessage(params)
		if err != nil {
			return err
		}
		response, err := json.Marshal(*message)
		if err != nil {
			return err
		}
		slog.Info("response", "body", string(response))
	}

	return nil
}
