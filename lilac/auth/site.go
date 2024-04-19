package auth

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
)

type SiteService struct {
	pb.UnimplementedSiteServer

	jwt      *crypto.Jwt
	aes      *crypto.Aes
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewSiteService(db *gorm.DB, aes *crypto.Aes, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *SiteService {
	return &SiteService{db: db, aes: aes, jwt: jwt, enforcer: enforcer}
}
