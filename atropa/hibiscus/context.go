package hibiscus

import (
	"encoding/json"
	"encoding/xml"
	"io"
	"log/slog"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
)

type Context struct {
	responseWriter http.ResponseWriter
	request        *http.Request
}

func (p *Context) Vars() map[string]string {
	return mux.Vars(p.request)
}

func (p *Context) ParseForm(form interface{}) error {
	err := p.request.ParseForm()
	if err != nil {
		return err
	}
	return gl_form_decoder.Decode(form, p.request.PostForm)
}

func (p *Context) HTML(status int, name string, data any) {
	if err := gl_html_template.ExecuteTemplate(p.responseWriter, name, data); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.write_header(status, TEXT_HTML_UTF8)
}
func (p *Context) Abort(status int, err error) {
	msg := err.Error()
	slog.Error(msg)
	p.String(status, msg)
}
func (p *Context) XML(status int, value any) {
	if err := xml.NewEncoder(p.responseWriter).Encode(value); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.write_header(status, APPLICATION_XML)
}
func (p *Context) JSON(status int, value any) {
	if err := json.NewEncoder(p.responseWriter).Encode(value); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.write_header(status, APPLICATION_JSON)
}
func (p *Context) String(status int, body string) {
	p.write_header(status, TEXT_PLAIN_UTF8)
	io.WriteString(p.responseWriter, body)
}
func (p *Context) Data(status int, content_type string, body []byte) {
	p.write_header(status, content_type)
	p.responseWriter.Write(body)
}
func (p *Context) write_header(status int, content_type string) {
	p.responseWriter.WriteHeader(status)
	p.responseWriter.Header().Set(HTTP_CONTENT_TYPE_HEADER, content_type)
}

func (p *Context) GetSession(name string) (*sessions.Session, error) {
	return gl_cookie_store.Get(p.request, name)
}

func (p *Context) SaveSession(session *sessions.Session) error {
	return session.Save(p.request, p.responseWriter)
}
