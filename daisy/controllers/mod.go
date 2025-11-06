package controllers

import (
	"bytes"
	"embed"
	"fmt"
	"html/template"
	"net/http"

	"github.com/casbin/casbin/v2"
	"github.com/gorilla/csrf"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
)

type Context struct {
	Theme    string
	Db       *gorm.DB
	Cache    *cache.RedisClient
	Enforcer *casbin.Enforcer
	Template *template.Template
	S3       *minio.Client
}

type Session struct{}

func NewSession(req *http.Request) *Session {

	// TODO
	return nil
}

type H map[string]interface{}

const (
	ContentTypeBinary = "application/octet-stream"
	ContentTypeForm   = "application/x-www-form-urlencoded"
	ContentTypeJson   = "application/json"
	ContentTypeHtml   = "text/html; charset=utf-8"
	ContentTypeText   = "text/plain; charset=utf-8"
	ContentTypeXml    = "application/rss+xml"

	ContentType   = "Content-Type"
	Authorization = "Authorization"
	Bearer        = "Bearer "
)

//go:embed themes/*/*.html
var gl_themes embed.FS

//go:embed assets/*/*
var gl_assets embed.FS

func Assets() http.Handler {
	return http.FileServerFS(gl_assets)
}
func LoadThemes(t *template.Template) (*template.Template, error) {
	return t.ParseFS(gl_themes)
}

func Html(ctx *Context, hnd func(*Context, *Session) (string, H, error)) http.HandlerFunc {
	return func(wrt http.ResponseWriter, req *http.Request) {
		name, data, err := hnd(ctx, NewSession(req))
		if err != nil {
			abort(wrt, err)
			return
		}
		data[csrf.TemplateTag] = csrf.TemplateField(req)
		var buf bytes.Buffer
		if err = ctx.Template.ExecuteTemplate(&buf, fmt.Sprintf("%s.%s", ctx.Theme, name), data); err != nil {
			abort(wrt, err)
			return
		}
		wrt.WriteHeader(http.StatusOK)
		wrt.Header().Set(ContentType, ContentTypeHtml)
		wrt.Header().Set("X-CSRF-Token", csrf.Token(req))
		wrt.Write(buf.Bytes())
	}
}

func Text(ctx *Context, hnd func(*Context, *Session) ([]byte, error)) http.HandlerFunc {
	return func(wrt http.ResponseWriter, req *http.Request) {
		body, err := hnd(ctx, NewSession(req))
		if err != nil {
			abort(wrt, err)
			return
		}
		wrt.WriteHeader(http.StatusOK)
		wrt.Header().Set(ContentType, ContentTypeText)
		wrt.Write(body)
	}
}

func Xml(ctx *Context, hnd func(*Context, *Session) (string, error)) http.HandlerFunc {
	return func(wrt http.ResponseWriter, req *http.Request) {
		body, err := hnd(ctx, NewSession(req))
		if err != nil {
			abort(wrt, err)
			return
		}
		wrt.WriteHeader(http.StatusOK)
		wrt.Header().Set(ContentType, ContentTypeXml)
		fmt.Fprintf(wrt, "%s", body)
	}
}
func abort(wrt http.ResponseWriter, err error) {
	wrt.WriteHeader(http.StatusInternalServerError)
	wrt.Header().Set(ContentType, ContentTypeText)
	fmt.Fprintf(wrt, "%s", err.Error())
}
