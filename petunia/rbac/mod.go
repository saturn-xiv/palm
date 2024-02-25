package rbac

import (
	"github.com/saturn-xiv/palm/petunia/env"
	"github.com/saturn-xiv/palm/petunia/env/redis"
)

type Config struct {
	Redis      redis.Cluster  `toml:"redis"`
	PostgreSql env.PostgreSql `toml:"postgresql"`
}
