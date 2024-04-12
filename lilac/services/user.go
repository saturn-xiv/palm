package services

import (
	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type UserService struct {
	pb.UnimplementedUserServer

	jwt      *crypto.Jwt
	mac      *crypto.HMac
	aes      *crypto.Aes
	enforcer *casbin.Enforcer
	db       *gorm.DB
	cache    *redis.Client
	queue    *rabbitmq.Config
	s3       *minio.Client
}

func NewUserService(db *gorm.DB, cache *redis.Client, aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt, enforcer *casbin.Enforcer, queue *rabbitmq.Config, s3 *minio.Client) *UserService {
	return &UserService{db: db, cache: cache, jwt: jwt, aes: aes, mac: mac, enforcer: enforcer, queue: queue, s3: s3}
}
