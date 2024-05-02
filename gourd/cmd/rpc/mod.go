package rpc

import (
	"github.com/saturn-xiv/palm/gourd/env"
	"github.com/saturn-xiv/palm/gourd/redis"
)

type Config struct {
	Namespace string        `toml:"namespace"`
	Redis     redis.Cluster `toml:"redis"`
	Database  env.Database  `toml:"database"`
}

type Ssl struct {
	CaFile   string
	KeyFile  string
	CertFile string
}
