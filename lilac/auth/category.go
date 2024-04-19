package auth

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

type CategoryService struct {
	pb.UnimplementedCategoryServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewCategoryService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *CategoryService {
	return &CategoryService{db: db, jwt: jwt, enforcer: enforcer}
}
