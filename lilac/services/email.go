package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type EmailService struct {
	pb.UnimplementedEmailServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	queue    *rabbitmq.Config
	enforcer *casbin.Enforcer
}

func NewEmailService(
	db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, queue *rabbitmq.Config) *EmailService {
	return &EmailService{db: db, jwt: jwt, queue: queue, enforcer: enforcer}
}
