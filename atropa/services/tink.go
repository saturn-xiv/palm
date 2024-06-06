package services

import (
	"context"
	"time"

	"google.golang.org/protobuf/types/known/emptypb"

	"github.com/saturn-xiv/palm/atropa/env/crypto"
	pb "github.com/saturn-xiv/palm/atropa/services/v2"
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

func NewAesService(aes *crypto.Aes) *AesService {
	return &AesService{aes: aes}
}

type AesService struct {
	pb.UnimplementedAesServer

	aes *crypto.Aes
}

func (p *AesService) Encrypt(ctx context.Context, req *pb.AesPlainMessage) (*pb.AesCodeMessage, error) {
	code, salt, err := p.aes.Encrypt(req.Payload)
	if err != nil {
		return nil, err
	}
	return &pb.AesCodeMessage{Payload: code, Salt: salt}, nil
}

func (p *AesService) Decrypt(ctx context.Context, req *pb.AesCodeMessage) (*pb.AesPlainMessage, error) {
	plain, err := p.aes.Decrypt(req.Payload, req.Salt)
	if err != nil {
		return nil, err
	}
	return &pb.AesPlainMessage{Payload: plain}, nil
}

// rpc () returns (AesMessage) {}
//
//	rpc Decrypt(AesMessage) returns (AesMessage) {}

func NewHmacService(hmac *crypto.HMac) *HmacService {
	return &HmacService{hmac: hmac}
}

type HmacService struct {
	pb.UnimplementedHMacServer

	hmac *crypto.HMac
}

func (p *HmacService) Sign(ctx context.Context, req *pb.HMacSignRequest) (*pb.HMacSignResponse, error) {
	code, err := p.hmac.Sign(req.Plain)
	if err != nil {
		return nil, err
	}
	return &pb.HMacSignResponse{Code: code}, nil
}
func (p *HmacService) Verify(ctx context.Context, req *pb.HMacVerifyRequest) (*emptypb.Empty, error) {
	if err := p.hmac.Verify(req.Code, req.Plain); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
