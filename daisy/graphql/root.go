package graphql

import (
	"gorm.io/gorm"

	"github.com/casbin/casbin/v3"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type Root struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	rabbitmq      *queue.RabbitMQ
	aead          *crypto.Aead
	hmac          *crypto.Hmac
	jwt           *crypto.Jwt
	enforcer      *casbin.Enforcer
	google_oauth2 GoogleOauth2Config
}

func NewRoot(db *gorm.DB, redis *cache.RedisClient, rabbitmq *queue.RabbitMQ,
	aead *crypto.Aead, hmac *crypto.Hmac, jwt *crypto.Jwt, enforcer *casbin.Enforcer,
	google_oauth2 GoogleOauth2Config) *Root {
	return &Root{db, redis, rabbitmq, aead, hmac, jwt, enforcer, google_oauth2}
}

func (p *Root) Query() *Query {
	return &Query{p.db, p.redis, p.rabbitmq, p.aead, p.hmac, p.jwt, p.enforcer, p.google_oauth2}
}

func (p *Root) Mutation() *Mutation {
	return &Mutation{p.db, p.redis, p.rabbitmq, p.aead, p.hmac, p.jwt, p.enforcer, p.google_oauth2}
}
