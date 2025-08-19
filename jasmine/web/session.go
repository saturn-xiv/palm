package web

import (
	"context"
	"net/http"
	"strings"

	"google.golang.org/grpc/metadata"
)

type Session struct {
	Token string
}

func SessionFromHttp(r *http.Request) *Session {
	return &Session{Token: parse_token_from_http_request(r)}
}

func SessionFromGrpc(c context.Context) *Session {
	return &Session{Token: parse_token_from_grpc_request(c)}
}

func parse_token_from_http_request(r *http.Request) string {
	{
		key := "token"
		params := r.URL.Query()
		if params.Has(key) {
			return params.Get(key)
		}
	}
	auth, ok := r.Header[HeaderAuthorization]
	if !ok {
		return ""
	}
	return parse_token_from_authorization(auth)
}

func parse_token_from_grpc_request(c context.Context) string {
	md, ok := metadata.FromIncomingContext(c)
	if !ok {
		return ""
	}
	auth, ok := md[strings.ToLower(HeaderAuthorization)]
	if !ok {
		return ""
	}
	return parse_token_from_authorization(auth)
}

func parse_token_from_authorization(auth []string) string {
	for _, it := range auth {
		if strings.HasPrefix(it, BearerTokenPrefix) {
			return strings.TrimPrefix(it, BearerTokenPrefix)
		}
	}
	return ""
}
