package services

import (
	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat "github.com/saturn-xiv/palm/atropa/env/wechat-pay"
	pb "github.com/saturn-xiv/palm/atropa/wechat/services/v2"
)

func NewPayService(redis *redis.Client, wechat *wechat.Config) *PayService {
	return &PayService{redis: redis, wechat: wechat}
}

type PayService struct {
	pb.UnimplementedPayServer

	wechat *wechat.Config
	redis  *redis.Client
}
