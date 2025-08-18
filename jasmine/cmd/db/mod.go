package db

import (
	"github.com/saturn-xiv/palm/jasmine/env"
)

type Config struct {
	Database env.Database `toml:"database"`
}
