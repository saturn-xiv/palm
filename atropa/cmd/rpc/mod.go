package rpc

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

type Config struct {
	Namespace string        `toml:"namespace"`
	Redis     redis.Cluster `toml:"redis"`
	Database  env.Database  `toml:"database"`
	Minio     env.Minio     `toml:"minio"`
}
