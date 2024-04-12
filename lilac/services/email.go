package services

import (
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type EmailService struct {
	pb.UnimplementedEmailServer

	jwt *crypto.Jwt
}

func NewEmailService(jwt *crypto.Jwt) *EmailService {
	return &EmailService{jwt: jwt}
}
