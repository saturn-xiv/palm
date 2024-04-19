package auth

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
)

type TagService struct {
	pb.UnimplementedTagServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewTagService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *TagService {
	return &TagService{db: db, jwt: jwt, enforcer: enforcer}
}
