package web

import (
	"encoding/json"
	"encoding/xml"
	h_template "html/template"
	"io"
	"log/slog"
	"net/http"
	t_template "text/template"
)

func JSON(w http.ResponseWriter, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeJSON)
	w.WriteHeader(s)
	return json.NewEncoder(w).Encode(o)
}

func Plain(w http.ResponseWriter, t *t_template.Template, n string, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeText)
	w.WriteHeader(s)
	return t.ExecuteTemplate(w, n, o)
}

func Abort(w http.ResponseWriter, e error, s int) {
	slog.Error(e.Error())
	w.Header().Set(HeaderContentType, ContentTypeText)
	w.WriteHeader(http.StatusInternalServerError)
	io.WriteString(w, e.Error())
}

func HTML(w http.ResponseWriter, t *h_template.Template, n string, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeText)
	w.WriteHeader(s)
	return t.ExecuteTemplate(w, n, o)
}

func XML(w http.ResponseWriter, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeXML)
	w.WriteHeader(s)
	return xml.NewEncoder(w).Encode(o)
}

func Redirect(w http.ResponseWriter, r *http.Request, u string, p bool) {
	if p {
		http.Redirect(w, r, u, http.StatusMovedPermanently)
	} else {
		http.Redirect(w, r, u, http.StatusFound)
	}
}
