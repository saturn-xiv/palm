package services

import (
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type SmsService struct {
	pb.UnimplementedSmsServer

	jwt   *crypto.Jwt
	queue *rabbitmq.Config
}

func NewSmsService(jwt *crypto.Jwt, queue *rabbitmq.Config) *SmsService {
	return &SmsService{jwt: jwt, queue: queue}
}
