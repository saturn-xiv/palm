package cmd

import (
	"fmt"

	"github.com/BurntSushi/toml"
	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	twilio_controllers "github.com/saturn-xiv/palm/petunia/controllers/twilio"
	"github.com/saturn-xiv/palm/petunia/env"
)

func launch_web_server(port int, config_file string) error {
	log.Debugf("load configuration from %s", config_file)
	config := env.Config{}
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	address := fmt.Sprintf("127.0.0.1:%d", port)
	router := gin.Default()

	twilio_router := router.Group("/twilio")
	twilio_controllers.Register(twilio_router)

	log.Infof("listen on http://%s", address)
	return router.Run(address)
}
