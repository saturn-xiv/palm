package controllers

import (
	"bytes"
	_ "embed"
	"text/template"
)

//go:embed templates/nginx.conf
var nginx_conf string

//go:embed templates/service.conf
var service_conf string

func Home(ctx *Context, ss *Session) (string, H, error) {
	// TODO
	return "home", H{}, nil
}

func NginxConf(ctx *Context, ss *Session) ([]byte, error) {
	tpl, err := template.New("").Parse(nginx_conf)
	if err != nil {
		return nil, err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, H{}); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func ServiceConf(ctx *Context, ss *Session) ([]byte, error) {
	tpl, err := template.New("").Parse(service_conf)
	if err != nil {
		return nil, err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, H{}); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}
