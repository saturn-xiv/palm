package services

import (
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type SmsService struct {
	pb.UnimplementedSmsServer

	jwt *crypto.Jwt
}

func NewSmsService(jwt *crypto.Jwt) *SmsService {
	return &SmsService{jwt: jwt}
}
