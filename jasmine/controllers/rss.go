package controllers

import (
	"net/http"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func Rss(ctx *Context) web.XmlHttpHandler {
	return func(r *http.Request) (interface{}, error) {
		// TODO
		return web.H{}, nil
	}
}
