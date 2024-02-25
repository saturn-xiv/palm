package s3

import (
	"context"

	pb "github.com/saturn-xiv/palm/petunia/s3/v2"
)

type S3Service struct {
	pb.UnimplementedS3Server
}

func NewS3Service() S3Service {
	return S3Service{}
}

func (p S3Service) Upload(_ctx context.Context, _req *pb.UploadRequest) (*pb.UploadResponse, error) {
	// TODO
	return &pb.UploadResponse{}, nil
}
func (p S3Service) PresignedUrl(_ctx context.Context, _req *pb.PresignedUrlRequest) (*pb.UrlResponse, error) {
	// TODO
	return &pb.UrlResponse{}, nil
}
func (p S3Service) PermanentUrl(_ctx context.Context, _req *pb.PermanentUrlRequest) (*pb.UrlResponse, error) {
	// TODO
	return &pb.UrlResponse{}, nil
}
