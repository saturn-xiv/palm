package seo

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func SitemapXmlByLang() gin.HandlerFunc {
	return func(c *gin.Context) {
		lang := c.Param("lang")
		// TODO
		c.HTML(http.StatusOK, "google/site-verify.html", gin.H{
			"title": "Hello palm",
			"lang":  lang,
		})
	}
}

func SitemapXml() gin.HandlerFunc {
	return func(c *gin.Context) {
		// TODO
		c.HTML(http.StatusOK, "google/site-verify.html", gin.H{
			"title": "Hello palm",
		})
	}
}
