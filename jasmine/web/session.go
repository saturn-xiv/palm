package web

import (
	"context"
	"net/http"
)

type Session struct{}

func (p *Session) FromHttp(r *http.Request) *Session {
	// TODO
	return nil
}

func (p *Session) FromGrpc(c context.Context) *Session {
	// TODO
	return nil
}
