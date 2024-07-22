package controllers

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/casbin/casbin/v2"
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam"
	"github.com/saturn-xiv/palm/atropa/daisy"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func Mount(router *gin.Engine, theme string, db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) error {
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
	if err := daisy.Mount(router, db, jwt); err != nil {
		return err
	}
	if err := balsam.Mount(router, gl_views_fs, theme, db, jwt); err != nil {
		return err
	}

	return nil
}
