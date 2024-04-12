package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type LeaveWordService struct {
	pb.UnimplementedLeaveWordServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func NewLeaveWordService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *LeaveWordService {
	return &LeaveWordService{db: db, jwt: jwt, enforcer: enforcer}
}
