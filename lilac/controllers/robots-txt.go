package controllers

import (
	"net/http"
	"text/template"

	"github.com/gin-gonic/gin"
)

// https://developers.google.com/search/docs/crawling-indexing/robots/create-robots-txt
func RobotsTxt() HandlerFunc {
	return func(c *gin.Context) error {
		tpl, err := template.ParseFS(gl_templates_fs, "templates/robots.txt")
		if err != nil {
			return err
		}
		if err = tpl.Execute(c.Writer, gin.H{
			"home": "https://change-me.org",
		}); err != nil {
			return err
		}

		c.Header(CONTENT_TYPE_HEADER, TEXT_CONTENT_TYPE)
		c.Status(http.StatusOK)
		return nil
	}
}
