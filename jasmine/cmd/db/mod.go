package db

import (
	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
)

type Config struct {
	SecretsStore string        `toml:"secrets-store"`
	Database     env.Database  `toml:"database"`
	Redis        redis.Cluster `toml:"redis"`
}
