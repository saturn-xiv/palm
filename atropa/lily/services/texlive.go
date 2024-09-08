package services

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	pb "github.com/saturn-xiv/palm/atropa/lily/services/v2"
	s3_pb "github.com/saturn-xiv/palm/atropa/s3/services/v2"
)

func NewTeXLiveService(s3 *minio.Client) *TeXLiveService {
	return &TeXLiveService{s3: s3}
}

type TeXLiveService struct {
	pb.UnimplementedTeXLiveServer

	s3 *minio.Client
}

func (p *TeXLiveService) ToPdf(ctx context.Context, req *pb.TeXLiveRequest) (*s3_pb.File, error) {
	// TODO
	return &s3_pb.File{}, nil
}
