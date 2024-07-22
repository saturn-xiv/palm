package services

import (
	"context"

	"github.com/saturn-xiv/palm/atropa/env/minio"
	pb "github.com/saturn-xiv/palm/atropa/s3/services/v2"
)

func NewS3Service(namespace string, client *minio.Client) *S3Service {
	return &S3Service{namespace: namespace, client: client}
}

type S3Service struct {
	pb.UnimplementedS3Server

	namespace string
	client    *minio.Client
}

func (p *S3Service) CreateBucket(ctx context.Context, req *pb.CreateBucketRequest) (*pb.CreateBucketResponse, error) {
	name, err := p.client.CreateBucket(ctx, req.Name, req.Public, int(req.ExpirationDays))
	if err != nil {
		return nil, err
	}
	return &pb.CreateBucketResponse{Name: name}, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *S3Service) Upload(ctx context.Context, req *pb.UploadRequest) (*pb.UploadResponse, error) {
	url, object, err := p.client.Upload(ctx, req.Bucket, req.Title, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &pb.UploadResponse{Url: url.String(), Object: object}, nil
}

func (p *S3Service) PermanentUrl(ctx context.Context, req *pb.PermanentUrlRequest) (*pb.UrlResponse, error) {
	url, err := p.client.PermanentUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType)
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}

func (p *S3Service) PresignedUrl(ctx context.Context, req *pb.PresignedUrlRequest) (*pb.UrlResponse, error) {
	url, err := p.client.PresignedUrl(ctx, req.Bucket, req.Object, req.Title, req.ContentType, req.Ttl.AsDuration())
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}
