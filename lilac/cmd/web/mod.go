package web

import (
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/redis"
)

type Config struct {
	Namespace string        `toml:"namespace"`
	Database  env.Database  `toml:"database"`
	Redis     redis.Cluster `toml:"redis"`
	Minio     env.Minio     `toml:"minio"`
}
