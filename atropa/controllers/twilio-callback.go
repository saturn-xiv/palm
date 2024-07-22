package controllers

import (
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/twilio/twilio-go/twiml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

type TwilioSmsStatusCallbackParams struct {
	Token string `uri:"token" binding:"required"`
}

func TwilioSmsStatusCallback(db *gorm.DB, jwt *crypto.Jwt) gin.HandlerFunc {
	return func(c *gin.Context) {
		var params TwilioSmsStatusCallbackParams
		if err := c.ShouldBindUri(&params); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}
		{
			_, subject, _, err := jwt.Verify(params.Token, env.JWT_ISSUER, "twilio")
			if err != nil {
				c.AbortWithError(http.StatusBadRequest, err)
				return
			}
			slog.Info("twilio callback", slog.String("subject", subject))
		}
		var msg = &twiml.MessagingMessage{}

		body := c.PostForm("Body")
		slog.Info("receive a message", slog.String("body", body))

		// TODO save message into database

		twiml, err := twiml.Messages([]twiml.Element{msg})
		if err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}

		c.Data(http.StatusOK, env.APPLICATION_XML, []byte(twiml))
	}
}
