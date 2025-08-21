package db

import (
	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
)

type Config struct {
	Database env.Database  `toml:"database"`
	Redis    redis.Cluster `toml:"redis"`
}
