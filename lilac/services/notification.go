package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type NotificationService struct {
	pb.UnimplementedNotificationServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewNotificationService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *NotificationService {
	return &NotificationService{db: db, jwt: jwt, enforcer: enforcer}
}
