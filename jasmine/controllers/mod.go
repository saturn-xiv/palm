package controllers

import (
	"encoding/json"
	"encoding/xml"
	h_template "html/template"
	"net/http"
	"text/template"

	"github.com/gorilla/sessions"
	"github.com/redis/go-redis/v9"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
)

const (
	ContentTypeJSON = "application/json; charset=utf-8"
	ContentTypeXML  = "application/xml"
	ContentTypeHTML = "text/html; charset=utf-8"
	ContentTypeText = "text/plain; charset=utf-8"
)

const (
	HeaderContentType = "Content-Type"
)

type Context struct {
	DB           *gorm.DB
	Redis        *redis.ClusterClient
	Session      sessions.Store
	Jwt          *crypto.Jwt
	TextTemplate *template.Template
	HTMLTemplate *h_template.Template
}

type HttpHandler func(w http.ResponseWriter, r *http.Request) error

func JSON(w http.ResponseWriter, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeJSON)
	w.WriteHeader(s)
	return json.NewEncoder(w).Encode(o)
}

func Plain(w http.ResponseWriter, t *template.Template, n string, o interface{}, s int) error {
	w.Header().Set(HeaderContentType, ContentTypeText)
	w.WriteHeader(s)
	return t.ExecuteTemplate(w, n, o)
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
