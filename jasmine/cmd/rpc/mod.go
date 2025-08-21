package rpc

import (
	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
)

type Config struct {
	SecretsStore      string                `toml:"secrets-store"`
	Redis             redis.Cluster         `toml:"redis"`
	Database          env.Database          `toml:"database"`
	Minio             env.Minio             `toml:"minio"`
	WechatPayMerchant env.WechatPayMerchant `toml:"wechat-pay-merchant"`
}
