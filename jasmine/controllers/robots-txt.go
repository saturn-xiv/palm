package controllers

import (
	"fmt"
	"net/http"

	"github.com/saturn-xiv/palm/jasmine/web"
)

// https://developers.google.com/search/docs/crawling-indexing/robots/intro
func RobotsTxt(r *http.Request) (string, interface{}, error) {
	// https://github.com/golang/go/issues/28940#issuecomment-485774726
	return "robots.txt", web.H{
		"home": fmt.Sprintf("https://%s", r.Host),
	}, nil
}
