package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	db       *gorm.DB
	jwt      *crypto.Jwt
	enforcer *casbin.Enforcer
}

func NewPolicyService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyService {
	return &PolicyService{db: db, jwt: jwt, enforcer: enforcer}
}
