package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type LocaleService struct {
	pb.UnimplementedLocaleServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewLocaleService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *LocaleService {
	return &LocaleService{db: db, jwt: jwt, enforcer: enforcer}
}
