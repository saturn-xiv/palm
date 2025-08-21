package controllers

import (
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func Home(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		// TODO get default lang from db
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
	// TODO redirect if setting.get "home.direct"(url, permanent)
	if true {
		return show_page(ctx, "tid", "oid")
	}
	// TODO
	return "home.html", web.H{
		"locale": lang,
	}, nil
}
