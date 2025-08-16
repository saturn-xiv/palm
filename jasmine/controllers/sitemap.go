package controllers

import (
	"net/http"

	"github.com/gorilla/mux"
)

func SitemapIndex(ctx *Context) HttpHandler {
	return func(w http.ResponseWriter, r *http.Request) error {
		// TODO
		return nil
	}
}
func SitemapByLang(ctx *Context) HttpHandler {
	return func(w http.ResponseWriter, r *http.Request) error {
		vars := mux.Vars(r)
		lang := vars["lang"]
		// TODO
		ctx.DB.Raw("select xxx", lang)
		return nil
	}
}
