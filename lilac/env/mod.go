package env

import (
	"github.com/saturn-xiv/palm/lilac/env/redis"
)

type Config struct {
	Redis      redis.Cluster `toml:"redis"`
	PostgreSql PostgreSql    `toml:"postgresql"`
	Minio      Minio         `toml:"minio"`
}
