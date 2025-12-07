package graphql

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
)

type Root struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	google_oauth2 GoogleOauth2Config
}

func NewRoot(db *gorm.DB, redis *cache.RedisClient, google_oauth2 GoogleOauth2Config) *Root {
	return &Root{db, redis, google_oauth2}
}

func (p *Root) Query() *Query {
	return &Query{p.db, p.redis, p.google_oauth2}
}

func (p *Root) Mutation() *Mutation {
	return &Mutation{p.db, p.redis, p.google_oauth2}
}
