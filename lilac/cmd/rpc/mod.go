package rpc

import (
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
)

type Config struct {
	Namespace         string                `toml:"namespace"`
	Redis             redis.Cluster         `toml:"redis"`
	Database          env.Database          `toml:"database"`
	Minio             env.Minio             `toml:"minio"`
	RabbitMq          rabbitmq.Config       `toml:"rabbitmq"`
	WeChatPayMerchant env.WechatPayMerchant `toml:"wechat-pay-merchant,omitempty"`
	WechatMiniProgram env.WechatMiniProgram `toml:"wechat-mini-program,omitempty"`
	WechatOauth2      env.WechatOauth2      `toml:"wechat-oauth2,omitempty"`
	GoogleOauth2      env.GoogleOauth2      `toml:"google-oauth2,omitempty"`
}
