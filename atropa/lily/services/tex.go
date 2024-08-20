package services

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	pb "github.com/saturn-xiv/palm/atropa/lily/services/v2"
)

func NewTexService(s3 *minio.Client) *TexService {
	return &TexService{s3: s3}
}

type TexService struct {
	pb.UnimplementedTexServer

	s3 *minio.Client
}

func (p *TexService) ToWord(ctx context.Context, req *pb.TexRequest) (*pb.File, error) {
	// TODO
	return &pb.File{}, nil
}

func (p *TexService) ToPdf(ctx context.Context, req *pb.TexRequest) (*pb.File, error) {
	// TODO
	return &pb.File{}, nil
}

func (p *TexService) Show(ctx context.Context, req *pb.ShowRequest) (*pb.ShowResponse, error) {
	// TODO
	return &pb.ShowResponse{}, nil
}

func (p *TexService) Status(ctx context.Context, req *pb.File) (*pb.StatusResponse, error) {
	// TODO
	return &pb.StatusResponse{}, nil
}
