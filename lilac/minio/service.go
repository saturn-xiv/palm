package minio

import (
	"context"
	"fmt"
	"net/url"
	"time"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
	log "github.com/sirupsen/logrus"
	"google.golang.org/protobuf/types/known/emptypb"

	pb "github.com/saturn-xiv/palm/lilac/minio/v2"
)

type S3Service struct {
	pb.UnimplementedS3Server

	client *minio.Client
}

func NewS3Service(client *minio.Client) *S3Service {
	return &S3Service{client: client}
}

func (p S3Service) CreateBucket(ctx context.Context, req *pb.CreateBucketRequest) (*emptypb.Empty, error) {
	found, err := p.client.BucketExists(ctx, req.Bucket)
	if err != nil {
		return nil, err
	}
	if !found {
		log.Infof("create bucket %s", req.Bucket)

		if err = p.client.MakeBucket(ctx, req.Bucket, minio.MakeBucketOptions{}); err != nil {
			return nil, err
		}
		if req.Published {
			log.Infof("set bucket %s to public", req.Bucket)
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
			`, now.Format("2006-01-02"), req.Bucket)
			log.Debugf("%s", policy)
			if err = p.client.SetBucketPolicy(ctx, req.Bucket, policy); err != nil {
				return nil, err
			}
		}

		if req.ExpirationDays != nil {
			log.Infof("set bucket %s expiration-days %d", req.Bucket, *req.ExpirationDays)
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

			if err = p.client.SetBucketLifecycle(ctx, req.Bucket, config); err != nil {
				return nil, err
			}
		}
	}

	return &emptypb.Empty{}, nil
}

func (p S3Service) UploadFile(ctx context.Context, req *pb.UploadFileRequest) (*pb.UploadFileResponse, error) {
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	url, err := p.client.PresignedPutObject(ctx, req.Bucket, req.Object, expiry)
	if err != nil {
		return nil, err
	}
	return &pb.UploadFileResponse{
		Url: url.String(),
	}, nil
}
func (p S3Service) GetPresignedUrl(ctx context.Context, req *pb.GetPresignedUrlRequest) (*pb.UrlResponse, error) {
	expiry := time.Second * time.Duration(req.Ttl.Seconds)
	params := make(url.Values)
	params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, req.Title))

	url, err := p.client.PresignedGetObject(ctx, req.Bucket, req.Object, expiry, params)
	if err != nil {
		return nil, err
	}
	return &pb.UrlResponse{Url: url.String()}, nil
}

func (p S3Service) GetPermanentUrl(_ctx context.Context, req *pb.GetPermanentUrlRequest) (*pb.UrlResponse, error) {
	return &pb.UrlResponse{
		Url: fmt.Sprintf("%s/%s/%s", p.client.EndpointURL(), req.Bucket, req.Bucket),
	}, nil
}
