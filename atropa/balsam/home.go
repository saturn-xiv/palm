package balsam

import (
	"bytes"
	"io/fs"
	"net/http"
	"text/template"

	"github.com/gin-gonic/gin"

	"github.com/saturn-xiv/palm/atropa/env"
)

func Home() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.HTML(http.StatusOK, "home.html.tpl", gin.H{
			"title": "Main website",
		})
	}
}

func RobotsTxt(views_fs fs.FS) gin.HandlerFunc {
	return func(c *gin.Context) {
		tpl, err := template.ParseFS(views_fs, "views/robots.txt.tpl")
		if err != nil {
			c.AbortWithError(http.StatusBadRequest, err)
			return
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, gin.H{}); err != nil {
			c.AbortWithError(http.StatusInternalServerError, err)
			return
		}

		c.Data(http.StatusOK, env.TEXT_PLAIN, buf.Bytes())
	}
}
