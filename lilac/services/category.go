package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
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
