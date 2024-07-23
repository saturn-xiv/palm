package services

import (
	"context"
	"errors"
	"strings"

	"google.golang.org/grpc/metadata"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

// https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md

func Auth(ctx context.Context, jwt *crypto.Jwt, audience string) error {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return errors.New("empty metadata")
	}

	for _, auth := range md.Get(env.GRPC_AUTHORIZATION_HEADER) {
		if strings.HasPrefix(auth, env.JWT_BEARER) {
			token := strings.TrimPrefix(auth, env.JWT_BEARER)
			if _, _, _, err := jwt.Verify(token, env.JWT_ISSUER, audience); err != nil {
				return err
			}
			return nil
		}
	}
	return errors.New("empty authorization header")
}
