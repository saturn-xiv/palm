package services

import (
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type EmailService struct {
	pb.UnimplementedEmailServer

	jwt   *crypto.Jwt
	queue *rabbitmq.Config
}

func NewEmailService(jwt *crypto.Jwt, queue *rabbitmq.Config) *EmailService {
	return &EmailService{jwt: jwt, queue: queue}
}
