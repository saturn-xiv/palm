package controllers

import (
	"bytes"
	"io/fs"
	"net/http"
	"text/template"

	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

func Home() hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		c.HTML(http.StatusOK, "home", hibiscus.H{
			"title": "Main website",
		})
	}
}

func RobotsTxt(views_fs fs.FS) hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		tpl, err := template.ParseFS(views_fs, "views/robots.txt.tpl")
		if err != nil {
			c.Abort(http.StatusBadRequest, err)
			return
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, hibiscus.H{}); err != nil {
			c.Abort(http.StatusInternalServerError, err)
			return
		}

		c.Data(http.StatusOK, hibiscus.TEXT_PLAIN_UTF8, buf.Bytes())
	}
}
