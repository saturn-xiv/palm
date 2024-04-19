package s3

import (
	"context"
	"fmt"
	"net/url"
	"time"

	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/s3/v2"
)

type Service struct {
	pb.UnimplementedS3Server

	namespace string
	db        *gorm.DB
	client    *minio.Client
	jwt       *crypto.Jwt
	enforcer  *casbin.Enforcer
}

func (p *Service) CreateBucket(ctx context.Context, req *pb.S3CreateBucketRequest) (*pb.S3CreateBucketResponse, error) {
	_, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}

	bucket, err := env.CreateBucket(ctx, p.client, p.namespace, req)
	if err != nil {
		return nil, err
	}
	return &pb.S3CreateBucketResponse{Name: bucket}, nil
}

func (p *Service) UploadFile(ctx context.Context, req *pb.S3UploadFileRequest) (*pb.S3UploadFileResponse, error) {
	_, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	url, err := p.client.PresignedPutObject(ctx, req.Bucket, req.Object, expiry)
	if err != nil {
		return nil, err
	}
	return &pb.S3UploadFileResponse{
		Url: url.String(),
	}, nil
}
func (p *Service) GetPresignedUrl(ctx context.Context, req *pb.S3GetPresignedUrlRequest) (*pb.S3UrlResponse, error) {
	_, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	public, err := pb.IsS3BucketPublic(req.Bucket)
	if err != nil {
		return nil, err
	}
	if public {
		return nil, status.Error(codes.InvalidArgument, "it is a public bucket")
	}
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	params := make(url.Values)
	params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, req.Title))

	url, err := p.client.PresignedGetObject(ctx, req.Bucket, req.Object, expiry, params)
	if err != nil {
		return nil, err
	}
	return &pb.S3UrlResponse{Url: url.String()}, nil
}

func (p *Service) GetPermanentUrl(ctx context.Context, req *pb.S3GetPermanentUrlRequest) (*pb.S3UrlResponse, error) {
	_, err := auth.NewCurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	public, err := pb.IsS3BucketPublic(req.Bucket)
	if err != nil {
		return nil, err
	}
	if !public {
		return nil, status.Error(codes.InvalidArgument, "it isn't a public bucket")
	}
	return &pb.S3UrlResponse{
		Url: fmt.Sprintf("%s/%s/%s", p.client.EndpointURL(), req.Bucket, req.Bucket),
	}, nil
}

func NewService(namespace string, db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, client *minio.Client) *Service {
	return &Service{namespace: namespace, db: db, client: client, jwt: jwt, enforcer: enforcer}
}
