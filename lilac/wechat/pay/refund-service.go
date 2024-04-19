package pay

import (
	"github.com/casbin/casbin/v2"
	"github.com/wechatpay-apiv3/wechatpay-go/core"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/wechat/pay/v2"
)

type RefundService struct {
	pb.UnimplementedWechatPayRefundServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
	merchant *core.Client
}

func NewRefundService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, merchant *core.Client) *RefundService {
	return &RefundService{db: db, jwt: jwt, enforcer: enforcer, merchant: merchant}
}
