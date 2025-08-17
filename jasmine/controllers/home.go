package controllers

import (
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func Home(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		// TODO get default lang
		lang := "en-US"
		return home(ctx, lang)
	}
}

func HomeByLang(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		vars := mux.Vars(r)
		return home(ctx, vars["lang"])
	}
}

func home(ctx *Context, lang string) (string, interface{}, error) {
	// TODO
	return "home.html", web.H{
		"locale": lang,
	}, nil
}
