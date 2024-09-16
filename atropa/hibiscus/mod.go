package hibiscus

import (
	"html/template"
	"net/url"

	"github.com/gorilla/schema"
	"github.com/gorilla/sessions"
)

type H = map[string]interface{}

// https://www.iana.org/assignments/media-types/media-types.xhtml
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
const (
	APPLICATION_GRPC_PROTO = "application/grpc+proto"
	APPLICATION_XML        = "application/xml"
	APPLICATION_JSON       = "application/json"
	APPLICATION_PDF        = "application/pdf"
	APPLICATION_DOC        = "application/msword"
	APPLICATION_XLS        = "application/vnd.ms-excel"
	APPLICATION_PPT        = "application/vnd.ms-powerpoint"
	APPLICATION_XLSX       = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
	APPLICATION_DOCX       = "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
	APPLICATION_PPTX       = "application/vnd.openxmlformats-officedocument.presentationml.presentation"
	TEXT_XML               = "text/xml"
	TEXT_PLAIN_UTF8        = "text/plain; charset=utf-8"
	TEXT_HTML_UTF8         = "text/html; charset=utf-8"

	JWT_ISSUER = "palm.atropa"
	JWT_BEARER = "Bearer "

	HTTP_CONTENT_TYPE_HEADER  = "Content-Type"
	HTTP_FORWARDED_FOR_HEADER = "X-Forwarded-For"
	HTTP_AUTHORIZATION_HEADER = "Authorization"
	HTTP_COOKIE_HEADER        = "Cookie"

	GRPC_AUTHORIZATION_HEADER   = "authorization"
	GRPC_ACCEPT_LANGUAGE_HEADER = "accept-language"
	GRPC_X_FORWARDED_FOR_HEADER = "x-forwarded-for"
)

var (
	gl_form_decoder  = schema.NewDecoder()
	gl_form_encoder  = schema.NewEncoder()
	gl_html_template = template.New("")
	gl_cookie_store  sessions.Store
)

func EncodeForm(form interface{}) (url.Values, error) {
	val := url.Values{}
	if err := gl_form_encoder.Encode(form, val); err != nil {
		return nil, err
	}
	return val, nil
}
