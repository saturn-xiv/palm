package rpc

import "github.com/saturn-xiv/palm/jasmine/env"

type Config struct {
	Minio env.Minio `toml:"minio"`
}

type Ssl struct {
	CaFile   string
	KeyFile  string
	CertFile string
}
