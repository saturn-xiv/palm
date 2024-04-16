package controllers

import (
	"embed"
	"errors"
	"html/template"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/minio/minio-go/v7"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/services"
	"gorm.io/gorm"
)

// https://www.iana.org/assignments/media-types/media-types.xhtml
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
const (
	ATOM_CONTENT_TYPE   = "application/atom+xml; charset=UTF-8"
	XML_CONTENT_TYPE    = "application/xml; charset=UTF-8"
	TEXT_CONTENT_TYPE   = "text/plain; charset=UTF-8"
	YAML_CONTENT_TYPE   = "application/yaml"
	CONTENT_TYPE_HEADER = "Content-Type"
)

type HandlerFunc = func(c *gin.Context) error

func Warp(f HandlerFunc) gin.HandlerFunc {
	return func(c *gin.Context) {
		if err := f(c); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
		}
	}
}

func NewCurrentUser(c *gin.Context, db *gorm.DB, jwt *crypto.Jwt) (*services.CurrentUser, error) {
	auth := c.GetHeader("Authorization")
	token, ok := strings.CutPrefix(auth, "Bearer ")
	if !ok {
		return nil, errors.New("authorization token is not provided")
	}
	return services.CurrentUserFromToken(token, db, jwt)
}

func Mount(router *gin.Engine, db *gorm.DB, jwt *crypto.Jwt, i18n *i18n.I18n, s3 *minio.Client) error {
	tpl, err := template.New("").ParseFS(gl_templates_fs, "templates/*")
	if err != nil {
		return err
	}
	router.SetHTMLTemplate(tpl)
	router.Use(env.GinLogger())
	router.Use(errorHandler)
	router.Use(gin.Recovery())

	router.POST("/attachments", Warp(AttachmentUpload(db, jwt, s3)))
	router.GET("/twilio/sms-status-callback/:token", Warp(TwilioSmsStatusCallback(db, jwt)))
	router.GET("/robots.txt", Warp(RobotsTxt()))
	router.GET("/sitemap.xml", Warp(SiteMap(db)))
	router.GET("/envoy.yaml", Warp(EnvoyYaml()))
	{
		group := router.Group("/:lang")
		group.GET("/sitemap.xml", Warp(SiteMapByLang(db)))
		group.GET("/rss.xml", Warp(RssByLang(db, i18n)))
	}

	router.StaticFS("/public", http.FS(gl_assets_fs))
	return nil
}

func errorHandler(c *gin.Context) {
	c.Next()

	if len(c.Errors) > 0 {
		for _, err := range c.Errors {
			slog.Error(err.Error())
		}
		status := c.Writer.Status()
		if status == 0 {
			status = http.StatusInternalServerError
		}
		c.JSON(status, c.Errors)
	}
}

//go:embed templates/*
var gl_templates_fs embed.FS

//go:embed assets/*
var gl_assets_fs embed.FS
