package graphql

import (
	"gorm.io/gorm"

	"github.com/casbin/casbin/v3"
	"github.com/minio/minio-go/v7"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/queue"
)

type Mutation struct {
	db            *gorm.DB
	redis         *cache.RedisClient
	rabbitmq      *queue.RabbitMQ
	aead          *crypto.Aead
	hmac          *crypto.Hmac
	jwt           *crypto.Jwt
	enforcer      *casbin.Enforcer
	minio         *minio.Client
	google_oauth2 GoogleOauth2Config
}
