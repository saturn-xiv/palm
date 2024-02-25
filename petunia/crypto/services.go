package crypto

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"

	pb "github.com/saturn-xiv/palm/petunia/crypto/v2"
	env_crypto "github.com/saturn-xiv/palm/petunia/env/crypto"
)

type JwtService struct {
	pb.UnimplementedJwtServer

	jwt *env_crypto.Jwt
}

func NewJwtService(jwt *env_crypto.Jwt) *JwtService {
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

	hmac *env_crypto.HMac
}

func NewHmacService(hmac *env_crypto.HMac) *HmacService {
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

	aes *env_crypto.Aes
}

func NewAesService(aes *env_crypto.Aes) *AesService {
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
