package pay

import (
	"github.com/casbin/casbin/v2"
	"github.com/wechatpay-apiv3/wechatpay-go/core"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/wechat/pay/v2"
)

type BillService struct {
	pb.UnimplementedWechatPayBillServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
	merchant *core.Client
}

func NewBillService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, merchant *core.Client) *BillService {
	return &BillService{db: db, jwt: jwt, enforcer: enforcer, merchant: merchant}
}
