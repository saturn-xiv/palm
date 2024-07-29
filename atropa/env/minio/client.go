package minio

import (
	"context"
	"fmt"
	"log/slog"
	"net/url"
	"path/filepath"
	"time"

	"github.com/google/uuid"
	minio_ "github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
)

type Client struct {
	client   *minio_.Client
	Readonly bool
}

func (p *Client) String() string {
	return p.client.EndpointURL().Host
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func (p *Client) CreateBucket(ctx context.Context, name string, public bool, expiration_days int) error {
	found, err := p.client.BucketExists(ctx, name)
	if err != nil {
		return err
	}
	if !found {
		slog.Info("create bucket", slog.String("name", name), slog.String("node", p.client.EndpointURL().Host))
		if err = p.client.MakeBucket(ctx, name, minio_.MakeBucketOptions{}); err != nil {
			return err
		}
		if public {
			slog.Info("set bucket to public")
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
			slog.Info("set bucket expiration", slog.Int("days", expiration_days))
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
func (p *Client) Upload(ctx context.Context, bucket string, title string, expires time.Duration) (*url.URL, string, error) {
	object := uuid.New().String()
	{
		ext := filepath.Ext(title)
		if ext != "" {
			object += ext
		}
	}

	url, err := p.client.PresignedPutObject(ctx, bucket, object, expires)
	if err != nil {
		return nil, "", err
	}
	return url, object, nil
}

func (p *Client) PermanentUrl(ctx context.Context, bucket string, object string, title string, content_type *string) (*url.URL, error) {
	params := make(url.Values)
	if content_type == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))
	} else {
		params.Set("response-content-type", *content_type)
	}
	it := p.client.EndpointURL().JoinPath(fmt.Sprintf("/%s/%s", bucket, object))
	it.RawQuery = params.Encode()
	return it, nil
}

func (p *Client) PresignedUrl(ctx context.Context, bucket string, object string, title string, content_type *string, expires time.Duration) (*url.URL, error) {
	params := make(url.Values)
	if content_type == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))
	} else {
		params.Set("response-content-type", *content_type)
	}
	return p.client.PresignedGetObject(ctx, bucket, object, expires, params)
}

func (p *Client) StatObject(ctx context.Context, bucket string, object string) (*minio_.ObjectInfo, error) {
	stat, err := p.client.StatObject(ctx, bucket, object, minio_.StatObjectOptions{})
	if err != nil {
		return nil, err
	}
	return &stat, nil
}
