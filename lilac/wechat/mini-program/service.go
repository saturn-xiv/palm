package mini_program

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/wechat/mini-program/v2"
)

type Service struct {
	pb.UnimplementedWechatMiniProgramServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *Service {
	return &Service{db: db, jwt: jwt, enforcer: enforcer}
}
