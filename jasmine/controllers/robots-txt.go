package controllers

import (
	"net/http"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func RobotsTxt(r *http.Request) (string, interface{}, error) {
	// TODO
	return "robots.txt", web.H{}, nil
}
