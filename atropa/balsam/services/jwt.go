package services

import (
	"context"
	"time"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

func NewJwtService(jwt *crypto.Jwt) *JwtService {
	return &JwtService{jwt: jwt}
}

type JwtService struct {
	pb.UnimplementedJwtServer

	jwt *crypto.Jwt
}

var (
	gl_jwt_extra_key = "ext"
)

func (p *JwtService) Sign(ctx context.Context, req *pb.JwtSignRequest) (*pb.JwtSignResponse, error) {
	claims := make(map[string]interface{})
	if req.Extra != nil {
		claims[gl_jwt_extra_key] = *req.Extra
	}
	not_before := time.Unix(req.NotBefore.Seconds, 0)
	expires_at := time.Unix(req.ExpiresAt.Seconds, 0)

	token, err := p.jwt.Sign(req.Issuer, req.Subject, req.Audiences, claims, &not_before, &expires_at)
	if err != nil {
		return nil, err
	}
	return &pb.JwtSignResponse{Token: token}, nil
}
func (p *JwtService) Verify(ctx context.Context, req *pb.JwtVerifyRequest) (*pb.JwtVerifyResponse, error) {
	jwt_id, subject, claims, err := p.jwt.Verify(req.Token, req.Issuer, req.Audience)
	if err != nil {
		return nil, err
	}
	var res pb.JwtVerifyResponse
	res.Subject = subject
	res.JwtId = jwt_id
	if it, ok := claims[gl_jwt_extra_key]; ok {
		if s, ok := it.(string); ok {
			res.Extra = &s
		}
	}
	return &res, nil
}
