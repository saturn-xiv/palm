package pay

import (
	"github.com/casbin/casbin/v2"
	"github.com/wechatpay-apiv3/wechatpay-go/core"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/wechat/pay/v2"
)

type NativeService struct {
	pb.UnimplementedWechatPayNativeServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
	merchant *core.Client
}

func NewNativeService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, merchant *core.Client) *NativeService {
	return &NativeService{db: db, jwt: jwt, enforcer: enforcer, merchant: merchant}
}
