package web

import (
	"log/slog"
	"net/http"

	h_template "html/template"
	t_template "text/template"
)

func WarpJson(h JsonHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		if e = JSON(w, o, http.StatusOK); e != nil {
			slog.Error(e.Error())
		}
	}
}

func WarpXml(h XmlHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		if e = XML(w, o, http.StatusOK); e != nil {
			slog.Error(e.Error())
		}
	}
}

func WarpPlain(t *t_template.Template, h PlainHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		n, o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		if e = Plain(w, t, n, o, http.StatusOK); e != nil {
			slog.Error(e.Error())
		}
	}
}

func WarpHtml(t *h_template.Template, h HtmlHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		n, o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		if e = HTML(w, t, n, o, http.StatusOK); e != nil {
			slog.Error(e.Error())
		}
	}
}
