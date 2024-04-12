package services

import (
	"context"
	"fmt"
	"net/url"
	"time"

	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
	log "github.com/sirupsen/logrus"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type S3Service struct {
	pb.UnimplementedS3Server

	namespace string
	db        *gorm.DB
	client    *minio.Client
	jwt       *crypto.Jwt
	enforcer  *casbin.Enforcer
}

func NewS3Service(namespace string, db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer, client *minio.Client) *S3Service {
	return &S3Service{namespace: namespace, db: db, client: client, jwt: jwt, enforcer: enforcer}
}

func (p S3Service) CreateBucket(ctx context.Context, req *pb.S3CreateBucketRequest) (*pb.S3CreateBucketResponse, error) {
	bucket, err := req.BucketName(p.namespace)
	if err != nil {
		return nil, err
	}
	found, err := p.client.BucketExists(ctx, bucket)
	if err != nil {
		return nil, err
	}
	if !found {
		log.Infof("create bucket %s", bucket)

		if err = p.client.MakeBucket(ctx, bucket, minio.MakeBucketOptions{}); err != nil {
			return nil, err
		}
		if req.Public {
			log.Infof("set bucket %s to public", bucket)
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
			`, now.Format("2006-01-02"), bucket)
			log.Debugf("%s", policy)
			if err = p.client.SetBucketPolicy(ctx, bucket, policy); err != nil {
				return nil, err
			}
		}

		if req.ExpirationDays != nil {
			log.Infof("set bucket %s expiration-days %d", bucket, *req.ExpirationDays)
			config := lifecycle.NewConfiguration()
			config.Rules = []lifecycle.Rule{
				{
					ID:     "expire-bucket",
					Status: "Enabled",
					Expiration: lifecycle.Expiration{
						Days: lifecycle.ExpirationDays(*req.ExpirationDays),
					},
				},
			}

			if err = p.client.SetBucketLifecycle(ctx, bucket, config); err != nil {
				return nil, err
			}
		}
	}

	return &pb.S3CreateBucketResponse{Name: bucket}, nil
}

func (p S3Service) UploadFile(ctx context.Context, req *pb.S3UploadFileRequest) (*pb.S3UploadFileResponse, error) {
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	url, err := p.client.PresignedPutObject(ctx, req.Bucket, req.Object, expiry)
	if err != nil {
		return nil, err
	}
	return &pb.S3UploadFileResponse{
		Url: url.String(),
	}, nil
}
func (p S3Service) GetPresignedUrl(ctx context.Context, req *pb.S3GetPresignedUrlRequest) (*pb.S3UrlResponse, error) {
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	params := make(url.Values)
	params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, req.Title))

	url, err := p.client.PresignedGetObject(ctx, req.Bucket, req.Object, expiry, params)
	if err != nil {
		return nil, err
	}
	return &pb.S3UrlResponse{Url: url.String()}, nil
}

func (p S3Service) GetPermanentUrl(_ctx context.Context, req *pb.S3GetPermanentUrlRequest) (*pb.S3UrlResponse, error) {
	return &pb.S3UrlResponse{
		Url: fmt.Sprintf("%s/%s/%s", p.client.EndpointURL(), req.Bucket, req.Bucket),
	}, nil
}
