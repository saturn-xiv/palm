package services

import (
	"context"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
)

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
