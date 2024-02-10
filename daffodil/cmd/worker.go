package cmd

import (
	"fmt"
	"os"

	log "github.com/sirupsen/logrus"

	gourd_tasks "github.com/saturn-xiv/palm/gourd/tasks"
	"github.com/saturn-xiv/palm/queue"
)

func launch_worker(rabbitmq string, queue_name string) error {
	hostname, err := os.Hostname()
	if err != nil {
		return err
	}
	consumer_name := fmt.Sprintf("%s-%d-%d", hostname, os.Getuid(), os.Getpid())
	log.Infof("start worker for %s", queue_name)
	queue := queue.NewRabbitMq(rabbitmq)

	queue.Consume(consumer_name, queue_name, &gourd_tasks.TwilioSmsConsumer{})
	queue.Consume(consumer_name, queue_name, &gourd_tasks.SmtpConsumer{})

	return fmt.Errorf("unknown queue %s", queue_name)
}
