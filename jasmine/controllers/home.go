package controllers

import (
	"context"
	"fmt"
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func Home(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		lang := "en-US"
		if buf, err := models.Get(ctx.DB, ctx.Aes, nil, models.KeySiteDefaultLanguage); err != nil {
			lang = string(buf)
		}
		return home(r.Context(), ctx, lang)
	}
}

func HomeByLang(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		vars := mux.Vars(r)
		return home(r.Context(), ctx, vars["lang"])
	}
}

func home(ctx_ context.Context, ctx *Context, lang string) (string, interface{}, error) {
	home_page := fmt.Sprintf("home.%s", lang)
	if buf, err := models.Get(ctx.DB, ctx.Aes, nil, models.KeySiteHomePage); err != nil {
		home_page = string(buf)
	}
	return show_page(ctx_, ctx, home_page)
}
