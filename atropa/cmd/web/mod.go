package web

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

type Config struct {
	KeysDir  string        `toml:"keys-dir"`
	Redis    redis.Cluster `toml:"redis"`
	Minio    minio.Config  `toml:"minio"`
	Database env.Database  `toml:"database"`
}
