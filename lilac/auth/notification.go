package auth

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
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
