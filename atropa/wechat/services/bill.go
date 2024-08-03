package services

import (
	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat "github.com/saturn-xiv/palm/atropa/env/wechat-pay"
	pb "github.com/saturn-xiv/palm/atropa/wechat/services/v2"
)

func NewPayBillService(redis *redis.Client, wechat *wechat.Config) *PayBillService {
	return &PayBillService{redis: redis, wechat: wechat}
}

type PayBillService struct {
	pb.UnimplementedPayBillServer

	wechat *wechat.Config
	redis  *redis.Client
}
