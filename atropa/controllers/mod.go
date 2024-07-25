package controllers

import (
	"embed"
	"fmt"
	"html/template"
	"net/http"

	"github.com/gin-gonic/gin"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/atropa/balsam"
	"github.com/saturn-xiv/palm/atropa/daisy"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

//go:embed views/*
var gl_views_fs embed.FS

//go:embed assets/* themes/jekyll-al-folio/assets/* themes/hugo-even/static/* themes/hugo-universal/static/*
var gl_assets_fs embed.FS

func Mount(router *gin.Engine, theme string, cache *redis.Client, jwt *crypto.Jwt, backend *grpc.ClientConn) error {
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
		handler, err := Graphql(jwt, backend)
		if err != nil {
			return err
		}
		router.GET("/graphql", gin.WrapH(handler))
		router.POST("/graphql", gin.WrapH(handler))
	}
	if err := daisy.Mount(router, jwt, backend); err != nil {
		return err
	}
	if err := balsam.Mount(router, gl_views_fs, theme, jwt, backend); err != nil {
		return err
	}

	return nil
}
