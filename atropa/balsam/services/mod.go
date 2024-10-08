package services

import (
	"context"
	"errors"
	"net"
	"strings"

	"github.com/go-playground/validator/v10"
	"golang.org/x/text/language"
	"google.golang.org/grpc/metadata"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/hibiscus"
)

var gl_validate *validator.Validate = validator.New()

// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md
func Auth(ctx context.Context, jwt *crypto.Jwt, audience string) error {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return errors.New("empty metadata")
	}
	for _, auth := range md.Get(hibiscus.GRPC_AUTHORIZATION_HEADER) {
		if strings.HasPrefix(auth, hibiscus.JWT_BEARER) {
			token := strings.TrimPrefix(auth, hibiscus.JWT_BEARER)
			if _, _, _, err := jwt.Verify(token, hibiscus.JWT_ISSUER, audience); err == nil {
				return nil
			}
		}
	}

	return errors.New("couldn't find jwt bearer header")
}

func Locale(ctx context.Context) language.Tag {
	md, ok := metadata.FromIncomingContext(ctx)
	if ok {
		for _, it := range md.Get(hibiscus.GRPC_ACCEPT_LANGUAGE_HEADER) {
			tag, err := language.Parse(it)
			if err == nil {
				return tag
			}
		}
	}
	return language.AmericanEnglish
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For#parsing
func ClientIP(ctx context.Context) net.IP {
	md, ok := metadata.FromIncomingContext(ctx)
	if ok {
		for _, it := range md.Get(hibiscus.GRPC_X_FORWARDED_FOR_HEADER) {
			for _, ips := range strings.Split(it, ",") {
				ip := net.ParseIP(ips)
				if ip != nil {
					return ip
				}
			}
		}
	}
	return net.IPv6loopback.To4()
}

func Token(ctx context.Context) (string, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return "", errors.New("empty metadata")
	}
	for _, auth := range md.Get(hibiscus.GRPC_AUTHORIZATION_HEADER) {
		if strings.HasPrefix(auth, hibiscus.JWT_BEARER) {
			token := strings.TrimPrefix(auth, hibiscus.JWT_BEARER)
			return token, nil
		}
	}
	return "", errors.New("couldn't find jwt bearer header")
}
