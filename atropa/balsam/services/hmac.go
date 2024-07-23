package services

import (
	"context"

	pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"google.golang.org/protobuf/types/known/emptypb"
)

func NewHmacService(hmac *crypto.HMac) *HmacService {
	return &HmacService{hmac: hmac}
}

type HmacService struct {
	pb.UnimplementedHMacServer

	hmac *crypto.HMac
}

func (p *HmacService) Sign(ctx context.Context, req *pb.HMacSignRequest) (*pb.HMacSignResponse, error) {
	code, err := p.hmac.Sign(req.Plain)
	if err != nil {
		return nil, err
	}
	return &pb.HMacSignResponse{Code: code}, nil
}
func (p *HmacService) Verify(ctx context.Context, req *pb.HMacVerifyRequest) (*emptypb.Empty, error) {
	if err := p.hmac.Verify(req.Code, req.Plain); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
