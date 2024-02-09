package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func ReactIntlLocales() gin.HandlerFunc {
	return func(c *gin.Context) {
		lang := c.Param("lang")
		// TODO
		c.HTML(http.StatusOK, "google/site-verify.html", gin.H{
			"title": "Hello palm",
			"lang":  lang,
		})
	}
}
