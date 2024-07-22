package services

import (
	"context"
	"fmt"
	"log/slog"
	"net/url"
	"path/filepath"
	"time"

	"github.com/google/uuid"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"

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
	name, err := req.Code(p.namespace)
	if err != nil {
		return nil, err
	}

	found, err := p.client.BucketExists(ctx, name)
	if err != nil {
		return nil, err
	}
	if !found {
		slog.Info(fmt.Sprintf("create bucket %s", name))

		if err = p.client.MakeBucket(ctx, name, minio.MakeBucketOptions{}); err != nil {
			return nil, err
		}
		if req.Public {
			slog.Info(fmt.Sprintf("set bucket %s to public", name))
			now := time.Now()
			policy := fmt.Sprintf(`
{
	"Version": "%s",
	"Statement": [
		{
			"Effect": "Allow",
			"Principal": {"AWS": "*"},
			"Action": [
				"s3:GetObject"
			],
			"Resource": "arn:aws:s3:::%s/*",
		},
	],
}
			`, now.Format("2006-01-02"), name)
			slog.Debug("set policy", slog.String("rule", policy))
			if err = p.client.SetBucketPolicy(ctx, name, policy); err != nil {
				return nil, err
			}
		}

		if req.ExpirationDays > 0 {
			slog.Info(fmt.Sprintf("set bucket %s expiration-days %d", name, req.ExpirationDays))
			config := lifecycle.NewConfiguration()
			config.Rules = []lifecycle.Rule{
				{
					ID:     "expire-bucket",
					Status: "Enabled",
					Expiration: lifecycle.Expiration{
						Days: lifecycle.ExpirationDays(req.ExpirationDays),
					},
				},
			}

			if err = p.client.SetBucketLifecycle(ctx, name, config); err != nil {
				return nil, err
			}
		}
	}
	return &pb.CreateBucketResponse{Name: name}, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *S3Service) Upload(ctx context.Context, req *pb.UploadRequest) (*pb.UploadResponse, error) {
	{
		_, err := p.bucket(req.Bucket)
		if err != nil {
			return nil, err
		}
	}
	expiry := time.Second * time.Duration(req.Ttl.Seconds)

	object := uuid.New().String()
	{
		ext := filepath.Ext(req.Title)
		if ext != "" {
			object += ext
		}
	}
	url, err := p.client.PresignedPutObject(ctx, req.Bucket, object, expiry)
	if err != nil {
		return nil, err
	}
	return &pb.UploadResponse{Url: url.String(), Object: object}, nil
}

func (p *S3Service) PermanentUrl(ctx context.Context, req *pb.PermanentUrlRequest) (*pb.UrlResponse, error) {
	{
		it, err := p.bucket(req.Bucket)
		if err != nil {
			return nil, err
		}
		if !it.Public {
			return nil, fmt.Errorf("bucket(%s) isn't public", req.Bucket)
		}
	}
	return &pb.UrlResponse{Url: fmt.Sprintf("%s/%s/%s", p.client.EndpointURL(), req.Bucket, req.Object)}, nil
}

func (p *S3Service) PresignedUrl(ctx context.Context, req *pb.PresignedUrlRequest) (*pb.UrlResponse, error) {
	{
		it, err := p.bucket(req.Bucket)
		if err != nil {
			return nil, err
		}
		if it.Public {
			return nil, fmt.Errorf("bucket(%s) is public", req.Bucket)
		}
	}
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	params := make(url.Values)

	if req.ContentType == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, req.Title))
	} else {
		params.Set("response-content-type", *req.ContentType)
	}

	url, err := p.client.PresignedGetObject(ctx, req.Bucket, req.Object, expiry, params)
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}

func (p *S3Service) bucket(code string) (*pb.CreateBucketRequest, error) {
	ns, it, err := pb.NewS3CreateBucketRequestFromCode(code)
	if err != nil {
		return nil, err
	}
	if ns != p.namespace {
		return nil, fmt.Errorf("namespace(%s) is not supported", ns)
	}
	return it, nil
}
