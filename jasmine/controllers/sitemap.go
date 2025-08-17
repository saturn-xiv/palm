package controllers

import (
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func SitemapIndex(ctx *Context) web.XmlHttpHandler {
	return func(r *http.Request) (interface{}, error) {
		// TODO
		return web.H{}, nil
	}
}
func SitemapByLang(ctx *Context) web.XmlHttpHandler {
	return func(r *http.Request) (interface{}, error) {
		vars := mux.Vars(r)
		lang := vars["lang"]
		// TODO
		ctx.DB.Raw("select xxx", lang)
		return web.H{}, nil
	}
}
