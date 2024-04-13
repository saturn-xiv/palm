package services

import (
	"github.com/casbin/casbin/v2"
	"github.com/wechatpay-apiv3/wechatpay-go/core"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type WechatPayService struct {
	pb.UnimplementedWechatPayServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
	merchant *core.Client
}

func NewWechatPayService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, merchant *core.Client) *WechatPayService {
	return &WechatPayService{db: db, jwt: jwt, enforcer: enforcer, merchant: merchant}
}
