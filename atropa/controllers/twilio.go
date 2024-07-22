package controllers

import (
	"errors"
	"fmt"
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/twilio/twilio-go/twiml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env"
)

type TwilioSmsStatusCallbackParams struct {
	Token string `uri:"token" binding:"required"`
}

func TwilioSmsStatusCallback(db *gorm.DB, token string) HandlerFunc {
	return func(c *gin.Context) error {
		var params TwilioSmsStatusCallbackParams
		if err := c.ShouldBindUri(&params); err != nil {
			return err
		}
		if params.Token != token {
			return errors.New("back token")
		}
		var msg = &twiml.MessagingMessage{}

		body := c.PostForm("Body")
		slog.Info(fmt.Sprintf("receive message: %s", body))

		twiml, err := twiml.Messages([]twiml.Element{msg})
		if err != nil {
			return err
		}
		c.Header(env.CONTENT_TYPE_HEADER, env.XML_CONTENT_TYPE)
		c.String(http.StatusOK, twiml)
		return nil
	}
}
