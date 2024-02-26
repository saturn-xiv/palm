package tink

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"

	crypto "github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/tink/v2"
)

type JwtService struct {
	pb.UnimplementedJwtServer

	jwt *crypto.Jwt
}

func NewJwtService(jwt *crypto.Jwt) *JwtService {
	return &JwtService{jwt: jwt}
}

func (p JwtService) Sign(_ctx context.Context, _req *pb.JwtSignRequest) (*pb.JwtSignResponse, error) {
	// TODO
	return &pb.JwtSignResponse{}, nil
}

func (p JwtService) Verify(_ctx context.Context, _req *pb.JwtVerifyRequest) (*pb.JwtVerifyResponse, error) {
	// TODO
	return &pb.JwtVerifyResponse{}, nil
}

type HmacService struct {
	pb.UnimplementedHmacServer

	hmac *crypto.HMac
}

func NewHmacService(hmac *crypto.HMac) *HmacService {
	return &HmacService{hmac: hmac}
}

func (p HmacService) Sign(_ctx context.Context, _req *pb.HmacSignRequest) (*pb.HmacSignResponse, error) {
	// TODO
	return &pb.HmacSignResponse{}, nil
}

func (p HmacService) Verify(_ctx context.Context, _req *pb.HmacVerifyRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

type AesService struct {
	pb.UnimplementedAesServer

	aes *crypto.Aes
}

func NewAesService(aes *crypto.Aes) *AesService {
	return &AesService{aes: aes}
}

func (p AesService) Encrypt(_ctx context.Context, _req *pb.PlainAesMessage) (*pb.CodeAesMessage, error) {
	// TODO
	return &pb.CodeAesMessage{}, nil
}

func (p AesService) Decrypt(_ctx context.Context, _req *pb.CodeAesMessage) (*pb.PlainAesMessage, error) {
	// TODO
	return &pb.PlainAesMessage{}, nil
}
