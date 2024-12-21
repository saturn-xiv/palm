package rpc

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

type Config struct {
	KeysDir      string          `toml:"keys-dir"`
	Database     env.Database    `toml:"database"`
	Redis        redis.Cluster   `toml:"redis"`
	RabbitMQ     rabbitmq.Config `toml:"rabbitmq"`
	Minio        minio.Config    `toml:"minio"`
	Tls          env.Tls         `toml:"tls"`
	GoogleOauth2 GoogleOauth2    `toml:"google-oauth2"`
}

type GoogleOauth2 struct {
	ProjectFile string `toml:"project-file"`
	RedirectURL string `toml:"redirect-url"`
}
