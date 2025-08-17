package web

import (
	"net/http"
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

type H map[string]interface{}

type JsonHttpHandler func(r *http.Request) (interface{}, error)
type XmlHttpHandler func(r *http.Request) (interface{}, error)
type PlainHttpHandler func(r *http.Request) (string, interface{}, error)
type HtmlHttpHandler func(r *http.Request) (string, interface{}, error)
