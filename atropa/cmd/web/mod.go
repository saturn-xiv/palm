package web

import "github.com/saturn-xiv/palm/atropa/env"

type Config struct {
	Theme    string       `toml:"theme"`
	Database env.Database `toml:"database"`
}
