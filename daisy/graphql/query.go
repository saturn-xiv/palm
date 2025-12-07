package graphql

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/env"
)

type Query struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	google_oauth2 GoogleOauth2Config
}

func (p *Query) Version() string { return env.Version() }
