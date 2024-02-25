package twilio

import (
	"net/http"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
	"github.com/twilio/twilio-go/twiml"
)

// https://www.twilio.com/docs/messaging/guides/track-outbound-message-status
func OutboundMessageStatusCallback() gin.HandlerFunc {
	return func(c *gin.Context) {
		// FIXME
		message_sid := c.PostForm("MessageSid")
		message_status := c.PostForm("MessageStatus")
		log.Infof("MessageSid: %s, MessageStatus: %s", message_sid, message_status)
		c.JSON(http.StatusOK, nil)
	}
}

// https://www.twilio.com/docs/messaging/tutorials/how-to-receive-and-reply/python
func MessagingReceive() gin.HandlerFunc {
	return func(c *gin.Context) {
		// FIXME
		body, ok := c.Params.Get("Body")
		if !ok {
			log.Errorf("empty message body")
		}
		log.Infof("receive sms: %s", body)
		response := twiml.MessagingMessage{}
		response.Body = body
		c.XML(http.StatusOK, response)
	}
}
