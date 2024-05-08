package rpc

import (
	"github.com/saturn-xiv/palm/gourd/env"
	"github.com/saturn-xiv/palm/gourd/env/redis"
)

type Config struct {
	Redis    redis.Cluster `toml:"redis"`
	Database env.Database  `toml:"database"`
}

type Ssl struct {
	CaFile   string
	KeyFile  string
	CertFile string
}
