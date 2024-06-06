package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

// https://www.iana.org/assignments/media-types/media-types.xhtml
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
const (
	PROTOBUF_CONTENT_TYPE = "application/grpc+proto"
	XML_CONTENT_TYPE      = "application/xml; charset=UTF-8"
	CONTENT_TYPE_HEADER   = "Content-Type"
)

type HandlerFunc = func(c *gin.Context) error

func Warp(f HandlerFunc) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := f(c); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
		}
	}
}

func Mount(router gin.IRouter, db *gorm.DB, twilio_validate_token string) error {
	{
		group := router.Group("/api/twilio")
		group.GET("/sms-status-callback/:token", Warp(TwilioSmsStatusCallback(db, twilio_validate_token)))
	}

	return nil
}
