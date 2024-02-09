package cmd

import (
	"fmt"
	"os"

	log "github.com/sirupsen/logrus"

	metasequoia_tasks "github.com/saturn-xiv/palm/metasequoia/tasks"
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

	queue.Consume(consumer_name, queue_name, &metasequoia_tasks.EmailConsumer{})

	return fmt.Errorf("unknown queue %s", queue_name)
}
