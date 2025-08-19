package casbin

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
)

type PolicyServer struct {
	v2.UnimplementedPolicyServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewPolicyServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyServer {
	return &PolicyServer{db: db, enforcer: enforcer, jwt: jwt}
}
