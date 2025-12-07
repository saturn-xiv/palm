package graphql

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
)

type Mutation struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	google_oauth2 GoogleOauth2Config
}
