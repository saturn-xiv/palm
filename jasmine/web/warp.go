package web

import (
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
		JSON(w, o, http.StatusOK)
	}
}

func WarpXml(h XmlHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		XML(w, o, http.StatusOK)
	}
}

func WarpPlain(t *t_template.Template, h PlainHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		n, o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		Plain(w, t, n, o, http.StatusOK)
	}
}

func WarpHtml(t *h_template.Template, h HtmlHttpHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		n, o, e := h(r)
		if e != nil {
			Abort(w, e, http.StatusInternalServerError)
			return
		}
		HTML(w, t, n, o, http.StatusOK)
	}
}
