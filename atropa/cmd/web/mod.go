package web

import "github.com/saturn-xiv/palm/atropa/env"

type Config struct {
	Database env.Database `toml:"database"`
}
