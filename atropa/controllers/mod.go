package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
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
	{
		handler, err := Graphql(db)
		if err != nil {
			return err
		}
		router.Any("/graphql", gin.WrapH(handler))
	}
	{
		router.Static("/3rd", "./node_modules")
	}

	return nil
}
