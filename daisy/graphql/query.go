package graphql

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type Query struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	rabbitmq      *queue.RabbitMQ
	aead          *crypto.Aead
	hmac          *crypto.Hmac
	jwt           *crypto.Jwt
	google_oauth2 GoogleOauth2Config
}

func (p *Query) Version() string { return env.Version() }
