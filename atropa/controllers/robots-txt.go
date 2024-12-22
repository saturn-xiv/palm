package controllers

import (
	"bytes"
	"embed"
	"net/http"
	"text/template"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

//go:embed robots.txt
var gl_robots_txt embed.FS

// https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt
func RobotsTxt(db *gorm.DB, jwt *crypto.Jwt) (hibiscus.HandlerFunc, error) {
	tpl, err := template.New("").
		ParseFS(gl_robots_txt,
			"robots.txt",
		)
	if err != nil {
		return nil, err
	}
	return func(c *hibiscus.Context) {
		var buf bytes.Buffer
		if err := tpl.ExecuteTemplate(&buf, "robots.txt", hibiscus.H{"host": c.Host()}); err != nil {
			c.Abort(http.StatusInternalServerError, err)
			return
		}
		c.PlainText(http.StatusOK, buf.String())
	}, nil
}
