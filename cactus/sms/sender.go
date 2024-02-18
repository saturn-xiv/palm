package sms

import (
	"context"
	"strings"
	"time"

	amqp "github.com/rabbitmq/amqp091-go"
	log "github.com/sirupsen/logrus"
	twilio "github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/cactus/env"
	pb "github.com/saturn-xiv/palm/cactus/sms/v2"
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

func (p *SendSmsConsumer) Consume(rabbitmq string, consumer_name string, queue_name string) error {
	con, err := amqp.Dial(rabbitmq)
	if err != nil {
		return err
	}
	defer con.Close()

	ch, err := con.Channel()
	if err != nil {
		return err
	}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	messages, err := ch.ConsumeWithContext(ctx, queue_name, consumer_name, true, false, false, false, nil)
	if err != nil {
		return err
	}

	for msg := range messages {
		log.Infof("receive message %s@%s", msg.MessageId, msg.ContentType)
		if err = p.consume(msg.MessageId, msg.ContentType, msg.Body); err != nil {
			return err
		}
	}
	return nil
}

func (p *SendSmsConsumer) consume(id string, content_type string, body []byte) error {
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
