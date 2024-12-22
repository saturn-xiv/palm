package hibiscus

import (
	"bytes"
	"embed"
	"encoding/json"
	"encoding/xml"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	text_template "text/template"

	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
)

type Context struct {
	responseWriter http.ResponseWriter
	request        *http.Request
}

func (p *Context) Host() string {
	return p.request.Host
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
	var buf bytes.Buffer
	if err := gl_html_template.ExecuteTemplate(&buf, name, data); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.Data(status, TEXT_HTML_UTF8, buf.Bytes())
}
func (p *Context) Abort(status int, err error) {
	msg := err.Error()
	slog.Error(msg)
	p.write_header(status, TEXT_PLAIN_UTF8)
	io.WriteString(p.responseWriter, msg)
}
func (p *Context) XML(status int, value any) {
	var buf bytes.Buffer
	if err := xml.NewEncoder(&buf).Encode(value); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.write_header(status, APPLICATION_XML)
	fmt.Fprintln(p.responseWriter, XML_HEADER)
	p.responseWriter.Write(buf.Bytes())
}
func (p *Context) JSON(status int, value any) {
	var buf bytes.Buffer
	if err := json.NewEncoder(&buf).Encode(value); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.Data(status, APPLICATION_JSON, buf.Bytes())

}
func (p *Context) PlainText(status int, fs *embed.FS, name string, value any) {
	tpl, err := text_template.New("").ParseFS(fs, name)
	if err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	var buf bytes.Buffer
	if err := tpl.ExecuteTemplate(&buf, name, value); err != nil {
		p.Abort(http.StatusInternalServerError, err)
		return
	}
	p.Data(status, TEXT_PLAIN_UTF8, buf.Bytes())
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
