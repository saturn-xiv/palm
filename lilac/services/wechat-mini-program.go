package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type WechatMiniProgramService struct {
	pb.UnimplementedWechatMiniProgramServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewWechatMiniProgramService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *WechatMiniProgramService {
	return &WechatMiniProgramService{db: db, jwt: jwt, enforcer: enforcer}
}
