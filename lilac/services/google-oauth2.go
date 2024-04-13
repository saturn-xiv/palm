package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type GoogleOauth2Service struct {
	pb.UnimplementedGoogleOauth2Server

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewGoogleOauth2Service(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *GoogleOauth2Service {
	return &GoogleOauth2Service{db: db, jwt: jwt, enforcer: enforcer}
}
