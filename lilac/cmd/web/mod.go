package web

import (
	"github.com/saturn-xiv/palm/lilac/env"
)

type Config struct {
	Namespace string       `toml:"namespace"`
	Database  env.Database `toml:"database"`
	Minio     env.Minio    `toml:"minio"`
}
