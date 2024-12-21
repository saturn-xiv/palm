package controllers

import (
	"log/slog"
	"net/http"

	"github.com/twilio/twilio-go/twiml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

type TwilioSmsStatusCallbackForm struct {
	Body string
}

// https://www.twilio.com/docs/usage/webhooks/messaging-webhooks
func TwilioSmsStatusCallback(db *gorm.DB, jwt *crypto.Jwt) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		vars := c.Vars()
		{
			_, subject, _, err := jwt.Verify(vars["token"], hibiscus.JWT_ISSUER, "twilio")
			if err != nil {
				c.Abort(http.StatusBadRequest, err)
				return
			}
			slog.Info("twilio callback", slog.String("subject", subject))
		}

		var form TwilioSmsStatusCallbackForm
		if err := c.ParseForm(&form); err != nil {
			c.Abort(http.StatusBadRequest, err)
			return
		}

		slog.Info("receive a message", slog.String("body", form.Body))

		// TODO save message
		var msg = &twiml.MessagingMessage{}
		twiml, err := twiml.Messages([]twiml.Element{msg})
		if err != nil {
			c.Abort(http.StatusInternalServerError, err)
			return
		}

		c.Data(http.StatusOK, hibiscus.APPLICATION_XML, []byte(twiml))
	}
}
