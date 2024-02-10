package twilio

import "github.com/gin-gonic/gin"

func Register(router gin.IRouter) {
	router.POST("/outbound-message-status", OutboundMessageStatusCallback())
	router.POST("/messaging-receive", MessagingReceive())
}
