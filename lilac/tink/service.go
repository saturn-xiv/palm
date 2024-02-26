package tink

import (
	"context"
	"time"

	"google.golang.org/protobuf/types/known/emptypb"

	crypto "github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/tink/v2"
)

type JwtService struct {
	pb.UnimplementedJwtServer

	client *crypto.Jwt
}

func NewJwtService(jwt *crypto.Jwt) *JwtService {
	return &JwtService{client: jwt}
}

func (p JwtService) Sign(_ctx context.Context, req *pb.JwtSignRequest) (*pb.JwtSignResponse, error) {
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	token, err := p.client.Sign(req.Issuer, req.Subject, req.Audience, req.Claims, expiry)
	if err != nil {
		return nil, err
	}
	return &pb.JwtSignResponse{Token: token}, nil
}

func (p JwtService) Verify(_ctx context.Context, req *pb.JwtVerifyRequest) (*pb.JwtVerifyResponse, error) {
	jwt_id, subject, claims, err := p.client.Verify(req.Token, req.Issuer, req.Audience)
	if err != nil {
		return nil, err
	}

	return &pb.JwtVerifyResponse{
		Id:      jwt_id,
		Subject: subject,
		Claims:  claims,
	}, nil
}

type HmacService struct {
	pb.UnimplementedHmacServer

	client *crypto.HMac
}

func NewHmacService(hmac *crypto.HMac) *HmacService {
	return &HmacService{client: hmac}
}

func (p HmacService) Sign(_ctx context.Context, req *pb.HmacSignRequest) (*pb.HmacSignResponse, error) {
	code, err := p.client.Sign(req.Plain)
	if err != nil {
		return nil, err
	}
	return &pb.HmacSignResponse{Code: code}, nil
}

func (p HmacService) Verify(_ctx context.Context, req *pb.HmacVerifyRequest) (*emptypb.Empty, error) {
	if err := p.client.Verify(req.Code, req.Plain); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

type AesService struct {
	pb.UnimplementedAesServer

	client *crypto.Aes
}

func NewAesService(aes *crypto.Aes) *AesService {
	return &AesService{client: aes}
}

func (p AesService) Encrypt(_ctx context.Context, req *pb.PlainAesMessage) (*pb.CodeAesMessage, error) {
	code, salt, err := p.client.Encrypt(req.Plain)
	if err != nil {
		return nil, err
	}
	return &pb.CodeAesMessage{Code: code, Salt: salt}, nil
}

func (p AesService) Decrypt(_ctx context.Context, req *pb.CodeAesMessage) (*pb.PlainAesMessage, error) {
	plain, err := p.client.Decrypt(req.Code, req.Salt)
	if err != nil {
		return nil, err
	}
	return &pb.PlainAesMessage{Plain: plain}, nil
}
