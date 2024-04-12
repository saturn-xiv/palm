package services

import (
	"github.com/casbin/casbin/v2"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type UserService struct {
	pb.UnimplementedUserServer

	jwt      *crypto.Jwt
	mac      *crypto.HMac
	aes      *crypto.Aes
	enforcer *casbin.Enforcer
}

func NewUserService(aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *UserService {
	return &UserService{jwt: jwt, aes: aes, mac: mac, enforcer: enforcer}
}
