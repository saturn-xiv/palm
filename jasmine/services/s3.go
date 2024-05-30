package services

import (
	"context"
	"fmt"
	"log/slog"
	"net/url"
	"time"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
)

type S3Handler struct {
	namespace string
	client    *minio.Client
}

func (p *S3Handler) CreateBucket(ctx context.Context, name string, public bool, expiration_days int32) error {
	name = p.namespace + "." + name
	found, err := p.client.BucketExists(ctx, name)
	if err != nil {
		return err
	}
	if !found {
		slog.Info(fmt.Sprintf("create bucket %s", name))

		if err = p.client.MakeBucket(ctx, name, minio.MakeBucketOptions{}); err != nil {
			return err
		}
		if public {
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
				return err
			}
		}

		if expiration_days > 0 {
			slog.Info(fmt.Sprintf("set bucket %s expiration-days %d", name, expiration_days))
			config := lifecycle.NewConfiguration()
			config.Rules = []lifecycle.Rule{
				{
					ID:     "expire-bucket",
					Status: "Enabled",
					Expiration: lifecycle.Expiration{
						Days: lifecycle.ExpirationDays(expiration_days),
					},
				},
			}

			if err = p.client.SetBucketLifecycle(ctx, name, config); err != nil {
				return err
			}
		}
	}
	return nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *S3Handler) UploadFile(ctx context.Context, bucket string, object string, ttl int32) (string, error) {
	expiry := time.Second * time.Duration(ttl)
	url, err := p.client.PresignedPutObject(ctx, bucket, object, expiry)
	if err != nil {
		return "", err
	}
	return url.String(), nil
}
func (p *S3Handler) GetPresignedURL(ctx context.Context, bucket string, object string, title string, ttl int32) (string, error) {
	expiry := time.Second * time.Duration(ttl)
	params := make(url.Values)
	params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))

	url, err := p.client.PresignedGetObject(ctx, bucket, object, expiry, params)
	if err != nil {
		return "", err
	}
	return url.String(), nil
}
func (p *S3Handler) GetPermanentURL(ctx context.Context, bucket string, object string) (string, error) {
	return fmt.Sprintf("%s/%s/%s", p.client.EndpointURL(), bucket, object), nil
}

func NewS3Handler(client *minio.Client, namespace string) *S3Handler {
	return &S3Handler{client: client, namespace: namespace}
}
