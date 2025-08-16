package rpc

import (
	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
)

type Config struct {
	Redis    redis.Cluster `toml:"redis"`
	Database env.Database  `toml:"database"`
}
