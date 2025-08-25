package controllers

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/gorilla/mux"

	portal_v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
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
	page := portal_v2.HtmlPage{Hash: hash}
	if err := ctx.Redis.GetJson(ctx_, &page); err != nil {
		return "", nil, err
	}
	slog.Debug("load page", slog.String("template", page.Template), slog.String("hash", hash))
	buf, err := page.Buffer()
	if err != nil {
		return "", nil, err
	}
	var data web.H
	if err := json.Unmarshal(buf, &data); err != nil {
		return "", nil, err
	}
	return page.Template, data, nil
}
