package cmd

import (
	"fmt"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	twilio_controllers "github.com/saturn-xiv/palm/petunia/controllers/twilio"
)

func launch_twilio_callback(port int) error {
	address := fmt.Sprintf("127.0.0.1:%d", port)
	router := gin.Default()

	twilio_router := router.Group("/twilio")
	twilio_controllers.Register(twilio_router)

	log.Infof("listen on http://%s", address)
	return router.Run(address)
}
