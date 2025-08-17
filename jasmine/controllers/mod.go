package controllers

import (
	"github.com/gorilla/sessions"
	"github.com/redis/go-redis/v9"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
)

type Context struct {
	DB      *gorm.DB
	Redis   *redis.ClusterClient
	Session sessions.Store
	Jwt     *crypto.Jwt
}
