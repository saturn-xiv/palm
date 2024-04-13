package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type WechatOauth2Service struct {
	pb.UnimplementedWechatOauth2Server

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewWechatOauth2Service(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *WechatOauth2Service {
	return &WechatOauth2Service{db: db, jwt: jwt, enforcer: enforcer}
}
