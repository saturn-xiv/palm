package cmd

import (
	"embed"
	"fmt"
	"html/template"
	"net/http"

	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"

	"github.com/saturn-xiv/palm/env"
	metasequoia_controllers "github.com/saturn-xiv/palm/metasequoia/controllers"
)

//go:embed assets/*
var gl_assets_fs embed.FS

//go:embed templates/*
var gl_templates_fs embed.FS

func launch_web_server(port int, config *env.Config) error {
	address := fmt.Sprintf("0.0.0.0:%d", port)
	router := gin.Default()

	tpl := template.Must(template.New("").ParseFS(gl_templates_fs, "templates/*/*.html"))
	router.SetHTMLTemplate(tpl)
	router.StaticFS("/public", http.FS(gl_assets_fs))

	router.GET("/robots.txt", metasequoia_controllers.RobotsTxt())
	router.GET("/sitemap.xml", metasequoia_controllers.SitemapXml())
	router.GET("/:lang/sitemap.xml", metasequoia_controllers.SitemapXmlByLang())
	router.GET("/:lang/rss.xml", metasequoia_controllers.RssXml())
	router.GET("/google.html", metasequoia_controllers.GoogleSiteVerify())
	router.GET("/baidu.html", metasequoia_controllers.BaiduSiteVerify())
	router.GET("/index-now.html", metasequoia_controllers.IndexNowSiteVerify())

	graphal_handler, err := metasequoia_controllers.Graphql()
	if err != nil {
		return err
	}
	router.Any("/graphql", gin.WrapH(graphal_handler))

	log.Infof("listen on http://%s", address)
	return router.Run(address)
}
