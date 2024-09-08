package web

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

type Config struct {
	Theme   string         `toml:"theme"`
	KeysDir string         `toml:"keys-dir"`
	Redis   redis.Cluster  `toml:"redis"`
	Minio   minio.Config   `toml:"minio"`
	Backend env.GrpcClient `toml:"backend"`
}
