package rpc

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat_mini_program "github.com/saturn-xiv/palm/atropa/env/wechat-mini-program"
	wechat_oauth2 "github.com/saturn-xiv/palm/atropa/env/wechat-oauth2"
	wechat_pay "github.com/saturn-xiv/palm/atropa/env/wechat-pay"
)

type Config struct {
	KeysDir           string                      `toml:"keys-dir"`
	GoogleOauth2      *GoogleOauth2               `toml:"google-oauth2,omitempty"`
	WechatOauth2      *wechat_oauth2.Config       `toml:"wechat-oauth2,omitempty"`
	WechatMiniProgram *wechat_mini_program.Config `toml:"wechat-mini-program,omitempty"`
	WechatPay         *wechat_pay.Config          `toml:"wechat-mini-pay,omitempty"`
	Redis             redis.Cluster               `toml:"redis"`
	Database          env.Database                `toml:"database"`
	RabbitMQ          rabbitmq.Config             `toml:"rabbitmq"`
	Minio             minio.Config                `toml:"minio"`
	Tls               env.Tls                     `toml:"tls"`
}

type GoogleOauth2 struct {
	ProjectID   string `toml:"project-id"`
	RedirectURL string `toml:"redirect-url"`
}