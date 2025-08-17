package controllers

import (
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func Home(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		// TODO
		return "home.html", web.H{}, nil
	}
}

func HomeByLang(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		vars := mux.Vars(r)
		// TODO
		return "home.html", web.H{
			"locale": vars["lang"],
		}, nil
	}
}
