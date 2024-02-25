package sms

import (
	"strings"

	log "github.com/sirupsen/logrus"
	twilio "github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/petunia/env"
	pb "github.com/saturn-xiv/palm/petunia/sms/v2"
)

type SendSmsConsumer struct {
	client *twilio.RestClient
	from   string
}

func NewSendSmsConsumer(config *env.Twilio) *SendSmsConsumer {
	return &SendSmsConsumer{
		client: config.Open(),
		from:   config.From,
	}
}

func (p *SendSmsConsumer) Handle(id string, content_type string, body []byte) error {
	var task pb.SendSmsTask
	if err := proto.Unmarshal(body, &task); err != nil {
		return err
	}
	log.Infof("send sms to %s: %s", strings.Join(task.To, ","), task.Message)

	for _, to := range task.To {
		params := twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Message)
		if task.Callback != nil {
			params.SetStatusCallback(*task.Callback)
		}

		response, err := p.client.Api.CreateMessage(&params)
		if err != nil {
			return err
		}
		log.Infof("Sid: %s", *response.Sid)
		log.Debugf("Status(%s) To(%s) Uri(%s)", *response.Status, *response.To, *response.Uri)
	}

	return nil
}
