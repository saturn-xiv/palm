package cmd

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/petunia/email"
	"github.com/saturn-xiv/palm/petunia/env"
)

func launch_send_email_consumer(queue_name string, config_file string) error {
	hostname, err := os.Hostname()
	if err != nil {
		return err
	}
	var config env.Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	consumer := email.NewSendEmailConsumer(config.Smtp.From(), config.Smtp.Open())
	return consumer.Consume(config.RabbitMq.URI(), fmt.Sprintf("consumer-%s-%d", hostname, os.Getpid()), queue_name)

}
