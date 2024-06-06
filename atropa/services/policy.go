package services

import (
	"github.com/casbin/casbin/v2"

	pb "github.com/saturn-xiv/palm/atropa/services/v2"
)

type PolicyService struct {
	pb.UnimplementedHMacServer

	enforcer *casbin.Enforcer
}
