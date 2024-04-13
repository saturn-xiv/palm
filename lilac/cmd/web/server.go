package web

import (
	"embed"
	"html/template"
	"io"
	"net/http"
	text_tpl "text/template"

	"github.com/BurntSushi/toml"
	"github.com/fvbock/endless"
	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	"github.com/saturn-xiv/palm/lilac/controllers"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

//go:embed templates/*
var gl_templates_fs embed.FS

//go:embed assets/*
var gl_assets_fs embed.FS

func Launch(address string, config_file string, keys_dir string) error {
	log.Debugf("load configuration from %s", config_file)
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}

	_, _, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

	tpl, err := template.New("").ParseFS(gl_templates_fs, "templates/*")
	if err != nil {
		return err
	}

	gin.DisableConsoleColor()
	router := gin.New()
	router.SetHTMLTemplate(tpl)
	router.Use(env.GinLogger())
	router.Use(errorHandler)
	router.Use(gin.Recovery())

	router.POST("/attachments", controllers.AttachmentUpload(db, jwt, s3))
	router.GET("/twilio/sms-status-callback/:token", controllers.TwilioSmsStatusCallback(db, jwt))
	router.GET("/robots.txt", RobotsTxt())
	router.GET("/sitemap.xml", controllers.SiteMap(db))
	{
		group := router.Group("/:lang")
		group.GET("/sitemap.xml", controllers.SiteMapByLang(db))
		group.GET("/rss.xml", controllers.RssByLang(db))
	}

	router.StaticFS("/public", http.FS(gl_assets_fs))

	log.Infof("listen on http://%s", address)
	endless.ListenAndServe(address, router)
	return nil
}

// https://developers.google.com/search/docs/crawling-indexing/robots/create-robots-txt
func RobotsTxt() gin.HandlerFunc {
	return func(c *gin.Context) {
		// TODO
		if err := robots_txt(c.Writer, "change-me"); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}
		c.Header(controllers.CONTENT_TYPE_HEADER, controllers.TEXT_CONTENT_TYPE)
		c.Status(http.StatusOK)
	}
}

func robots_txt(writer io.Writer, domain string) error {
	tpl, err := text_tpl.ParseFS(gl_templates_fs, "templates/robots.txt")
	if err != nil {

		return err
	}
	return tpl.Execute(writer, gin.H{
		"domain": domain,
	})
}

func errorHandler(c *gin.Context) {
	c.Next()

	if len(c.Errors) > 0 {
		for _, err := range c.Errors {
			log.Errorf("%s", err)
		}
		status := c.Writer.Status()
		if status == 0 {
			status = http.StatusInternalServerError
		}
		c.JSON(status, c.Errors)
	}
}
