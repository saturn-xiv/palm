package controllers

import (
	"log/slog"
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func ShowPage(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		vars := mux.Vars(r)
		tid := vars["tid"]
		oid := vars["oid"]
		return show_page(ctx, tid, oid)
	}
}

func show_page(ctx *Context, tid string, oid string) (string, interface{}, error) {
	slog.Debug("load page", slog.String("template", tid), slog.String("object", oid))
	// TODO
	return tid, web.H{}, nil
}
