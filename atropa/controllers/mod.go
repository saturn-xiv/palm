package controllers

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router *gin.Engine, theme string, db *gorm.DB, jwt *crypto.Jwt) error {
	{
		tpl, err := template.New("").ParseFS(gl_views_fs, "views/*.tpl", fmt.Sprintf("views/%s/*.tpl", theme))
		if err != nil {
			return err
		}
		router.SetHTMLTemplate(tpl)
	}
	{
		router.Static("/3rd", "./node_modules")
		router.StaticFS("/public", http.FS(gl_assets_fs))
	}
	{
		handler, err := Graphql(db)
		if err != nil {
			return err
		}
		router.GET("/graphql", gin.WrapH(handler))
		router.POST("/graphql", gin.WrapH(handler))
	}
	{
		group := router.Group("/api/twilio")
		group.GET("/sms-status-callback/:token", TwilioSmsStatusCallback(db, jwt))
	}
	{
		router.GET("/robots.txt", RobotsTxt())
		router.GET("/", Home())
	}

	return nil
}
