package services

import (
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	jwt *crypto.Jwt
}

func NewPolicyService(jwt *crypto.Jwt) *PolicyService {
	return &PolicyService{jwt: jwt}
}
