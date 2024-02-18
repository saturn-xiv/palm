package cmd

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/cactus/env"
	"github.com/saturn-xiv/palm/cactus/sms"
)

func launch_send_sms_consumer(queue_name string, config_file string) error {
	hostname, err := os.Hostname()
	if err != nil {
		return err
	}
	var config env.Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	consumer := sms.NewSendSmsConsumer(&config.Twilio)
	return consumer.Consume(config.RabbitMq.URI(), fmt.Sprintf("twilio-sms-consumer-%s-%d", hostname, os.Getpid()), queue_name)

}
