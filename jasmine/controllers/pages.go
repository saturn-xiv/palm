package controllers

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func ShowPage(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		vars := mux.Vars(r)
		hash := vars["hash"]
		return show_page(r.Context(), ctx, hash)
	}
}

func show_page(ctx_ context.Context, ctx *Context, hash string) (string, interface{}, error) {
	page, err := GetHtmlPage(ctx_, ctx.Redis, hash)
	if err != nil {
		return "", nil, err
	}
	slog.Debug("load page", slog.String("template", page.Template), slog.String("object", hash))
	var data web.H
	if err := json.Unmarshal(page.Data, &data); err != nil {
		return "", nil, err
	}
	return page.Template, data, nil
}
