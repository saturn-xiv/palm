package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
	"github.com/twilio/twilio-go/twiml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

func TwilioSmsStatusCallback(db *gorm.DB, jwt *crypto.Jwt) HandlerFunc {
	return func(c *gin.Context) error {
		// TODO verify token
		var msg = &twiml.MessagingMessage{}

		body := c.PostForm("Body")
		log.Infof("receive message: %s", body)

		twiml, err := twiml.Messages([]twiml.Element{msg})
		if err != nil {
			return err
		}
		c.Header(CONTENT_TYPE_HEADER, XML_CONTENT_TYPE)
		c.String(http.StatusOK, twiml)
		return nil
	}
}
