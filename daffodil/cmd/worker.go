package cmd

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"
	log "github.com/sirupsen/logrus"

	"github.com/saturn-xiv/palm/env"
	gourd_tasks "github.com/saturn-xiv/palm/gourd/tasks"
	gourd_pb "github.com/saturn-xiv/palm/gourd/v2"
	"github.com/saturn-xiv/palm/queue"
)

func launch_worker(config_file string, job_name string, queue_name string) error {
	hostname, err := os.Hostname()
	if err != nil {
		return err
	}
	consumer_name := fmt.Sprintf("%s-%d-%d", hostname, os.Getuid(), os.Getpid())
	log.Infof("start worker for %s", queue_name)

	if job_name == env.QueueName((*gourd_pb.SendEmail)(nil)) {
		var config gourd_tasks.SmtpConsumerConfig
		if _, err = toml.DecodeFile(config_file, &config); err != nil {
			return err
		}
		queue := queue.NewRabbitMq(config.RabbitMq.URI())
		handler := config.Open()
		return queue.Consume(consumer_name, queue_name, &handler)
	}
	if job_name == env.QueueName((*gourd_pb.SendSms)(nil)) {
		var config gourd_tasks.SmsConsumerConfig
		if _, err = toml.DecodeFile(config_file, &config); err != nil {
			return err
		}
		queue := queue.NewRabbitMq(config.RabbitMq.URI())
		return queue.Consume(consumer_name, queue_name, &gourd_tasks.TwilioSmsConsumerHandler{})
	}
	return fmt.Errorf("unknown queue %s", queue_name)
}
