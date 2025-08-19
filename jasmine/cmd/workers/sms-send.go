package workers

import (
	"context"
	"log/slog"

	"github.com/BurntSushi/toml"
	"github.com/twilio/twilio-go"
	twilio_api "github.com/twilio/twilio-go/rest/api/v2010"
	"google.golang.org/protobuf/proto"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/rabbitmq"
	v2 "github.com/saturn-xiv/palm/jasmine/services/sms/v2"
)

func LaunchSmsSendConsumer(config_file string, queue string) error {
	slog.Info("start sms-send consumer", slog.String("queue", queue))
	slog.Debug("load configuration", slog.String("file", config_file))
	ctx := context.Background()
	var config sendSmsWorkerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	consumer := newSendSmsWorker(config.Twilio.Open(), config.Twilio.From)
	return config.RabbitMQ.Consume(ctx, queue, consumer)
}

type sendSmsWorkerConfig struct {
	Twilio   env.Twilio      `toml:"twilio"`
	RabbitMQ rabbitmq.Config `toml:"rabbitmq"`
}

type sendSmsWorker struct {
	client *twilio.RestClient
	from   string
}

func newSendSmsWorker(client *twilio.RestClient, from string) *sendSmsWorker {
	return &sendSmsWorker{client: client, from: from}
}

func (p *sendSmsWorker) Handle(id string, content_type string, message []byte) error {
	var task v2.SmsSendTask
	if err := proto.Unmarshal(message, &task); err != nil {
		return err
	}

	for _, to := range task.To {
		params := &twilio_api.CreateMessageParams{}
		params.SetTo(to)
		params.SetFrom(p.from)
		params.SetBody(task.Message)
		if task.CallbackUrl != nil {
			params.SetStatusCallback(*task.CallbackUrl)
		}
		slog.Info("send sms", slog.String("message", task.Message), slog.String("to", to))
		res, err := p.client.Api.CreateMessage(params)
		if err != nil {
			slog.Error("", slog.Int("code", *res.ErrorCode), slog.String("message", *res.ErrorMessage))
			return err
		}
	}
	return nil
}
