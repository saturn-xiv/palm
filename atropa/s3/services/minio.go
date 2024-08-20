package services

import (
	"context"

	"google.golang.org/protobuf/types/known/emptypb"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	pb "github.com/saturn-xiv/palm/atropa/s3/services/v2"
)

func NewS3Service(cluster *minio.Client) *S3Service {
	return &S3Service{cluster: cluster}
}

type S3Service struct {
	pb.UnimplementedS3Server

	cluster *minio.Client
}

func (p *S3Service) CreateBucket(ctx context.Context, req *pb.CreateBucketRequest) (*pb.CreateBucketResponse, error) {
	name, err := p.cluster.CreateBucket(ctx, req.Name, req.Public, int(req.ExpirationDays))
	if err != nil {
		return nil, err
	}
	return &pb.CreateBucketResponse{Name: name}, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *S3Service) UploadObject(ctx context.Context, req *pb.UploadObjectRequest) (*pb.UploadObjectResponse, error) {
	url, object, err := p.cluster.Upload(ctx, req.Bucket, req.Title, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &pb.UploadObjectResponse{Url: url.String(), Object: object}, nil
}

func (p *S3Service) ObjectPermanentUrl(ctx context.Context, req *pb.ObjectPermanentUrlRequest) (*pb.UrlResponse, error) {
	url, err := p.cluster.PermanentUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType)
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}

func (p *S3Service) ObjectPresignedUrl(ctx context.Context, req *pb.ObjectPresignedUrlRequest) (*pb.UrlResponse, error) {
	url, err := p.cluster.PresignedUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}

func (p *S3Service) RemoveObject(ctx context.Context, req *pb.RemoveObjectRequest) (*emptypb.Empty, error) {
	if err := p.cluster.RemoveObject(ctx, req.Bucket, req.Object); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
