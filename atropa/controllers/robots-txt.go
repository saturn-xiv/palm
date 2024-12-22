package controllers

import (
	"embed"
	"net/http"

	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

//go:embed robots.txt
var gl_robots_txt embed.FS

// https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt
func RobotsTxt() hibiscus.HandlerFunc {
	return func(c *hibiscus.Context) {
		c.PlainText(http.StatusOK, &gl_robots_txt, "robots.txt", hibiscus.H{"host": c.Host()})
	}
}
