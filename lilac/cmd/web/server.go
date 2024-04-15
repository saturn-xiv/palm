package web

import (
	"context"
	"embed"
	"errors"
	"fmt"
	"html/template"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	text_tpl "text/template"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/gin-gonic/gin"

	"github.com/saturn-xiv/palm/lilac/controllers"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
)

//go:embed templates/*
var gl_templates_fs embed.FS

//go:embed assets/*
var gl_assets_fs embed.FS

func Launch(address string, config_file string, keys_dir string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
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
	i18n, err := i18n.New(db)
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

	router.POST("/attachments", controllers.Warp(controllers.AttachmentUpload(db, jwt, s3)))
	router.GET("/twilio/sms-status-callback/:token", controllers.Warp(controllers.TwilioSmsStatusCallback(db, jwt)))
	router.GET("/robots.txt", controllers.Warp(robotsTxt()))
	router.GET("/sitemap.xml", controllers.Warp(controllers.SiteMap(db)))
	{
		group := router.Group("/:lang")
		group.GET("/sitemap.xml", controllers.Warp(controllers.SiteMapByLang(db)))
		group.GET("/rss.xml", controllers.Warp(controllers.RssByLang(db, i18n)))
	}

	router.StaticFS("/public", http.FS(gl_assets_fs))

	slog.Info(fmt.Sprintf("listen on http://%s", address))
	server := &http.Server{
		Addr:    address,
		Handler: router,
	}
	go func() {
		if err := server.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			slog.Error(err.Error())
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	slog.Warn("shutting down http server...")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		return err
	}

	slog.Info("http server exiting")
	return nil
}

// https://developers.google.com/search/docs/crawling-indexing/robots/create-robots-txt
func robotsTxt() controllers.HandlerFunc {
	return func(c *gin.Context) error {
		tpl, err := text_tpl.ParseFS(gl_templates_fs, "templates/robots.txt")
		if err != nil {
			return err
		}
		if err = tpl.Execute(c.Writer, gin.H{
			"home": "https://change-me.org",
		}); err != nil {
			return err
		}

		c.Header(controllers.CONTENT_TYPE_HEADER, controllers.TEXT_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
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
