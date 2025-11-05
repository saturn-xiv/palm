package sms

import (
	"encoding/json"
	"log/slog"

	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	v2 "com.github/saturn_xiv/palm/daisy/sms/v2"
)

type Twilio struct {
	AccountSid string
	AuthToken  string
}

func (p *Twilio) Open(from string, to string, body string) *twilio.RestClient {
	return twilio.NewRestClientWithParams(twilio.ClientParams{
		Username: p.AccountSid,
		Password: p.AuthToken,
	})
}

func send(client *twilio.RestClient, from string, to string, body string) error {
	slog.Debug("send sms message", "from", from, "to", to, "body", body)
	params := &twilio_api.CreateMessageParams{}
	params.SetTo(to)
	params.SetFrom(from)
	params.SetBody(body)

	message, err := client.Api.CreateMessage(params)
	if err != nil {
		return err
	}
	response, err := json.Marshal(*message)
	if err != nil {
		return err
	}
	slog.Info("response", "body", string(response))

	return nil
}

type TwilioSmsProtobufConsumer struct {
	client *twilio.RestClient
}

func NewTwilioSmsProtobufConsumer(client *twilio.RestClient) *TwilioSmsProtobufConsumer {
	return &TwilioSmsProtobufConsumer{client: client}
}

func (p *TwilioSmsProtobufConsumer) Name() string {
	return "twilio-sms.protobuf"
}
func (p *TwilioSmsProtobufConsumer) Execute(id string, content_type string, body []byte) error {
	var task v2.Task
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	for _, to := range task.To {
		if err := send(p.client, task.From, to, task.Body); err != nil {
			return err
		}
	}
	return nil
}
