package controllers

import (
	"log/slog"
	"net/http"

	"github.com/gorilla/mux"

	"github.com/saturn-xiv/palm/jasmine/web"
)

func ShowPage(ctx *Context) web.HtmlHttpHandler {
	return func(r *http.Request) (string, interface{}, error) {
		// TODO
		vars := mux.Vars(r)
		tid := vars["tid"]
		oid := vars["oid"]
		slog.Debug("load page", slog.String("template", tid), slog.String("object", oid))
		return tid, web.H{}, nil
	}
}
