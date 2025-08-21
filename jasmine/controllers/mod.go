package controllers

import (
	"github.com/gorilla/sessions"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/env/redis"
)

type Context struct {
	DB      *gorm.DB
	Redis   *redis.Client
	Session sessions.Store
	Jwt     *crypto.Jwt
}
