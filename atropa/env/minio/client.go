package minio

import (
	"context"
	"errors"
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
	namespace string
	nodes     []*node
}

type node struct {
	client   *minio_.Client
	readonly bool
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func (p *Client) CreateBucket(ctx context.Context, name string, public bool, expiration_days int) (string, error) {
	client, err := p.writable()
	if err != nil {
		return "", err
	}
	bucket_ := bucket{
		namespace:       p.namespace,
		name:            name,
		public:          public,
		expiration_days: expiration_days,
	}
	bucket, err := bucket_.code()
	if err != nil {
		return "", err
	}

	found, err := client.BucketExists(ctx, bucket)
	if err != nil {
		return "", err
	}
	if !found {
		slog.Info("create bucket", slog.String("name", bucket), slog.String("node", client.EndpointURL().Host))
		if err = client.MakeBucket(ctx, bucket, minio_.MakeBucketOptions{}); err != nil {
			return "", err
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
			`, now.Format("2006-01-02"), bucket)
			slog.Debug("set policy", slog.String("rule", policy))
			if err = client.SetBucketPolicy(ctx, bucket, policy); err != nil {
				return "", err
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

			if err = client.SetBucketLifecycle(ctx, bucket, config); err != nil {
				return "", err
			}
		}
	}
	return bucket, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *Client) Upload(ctx context.Context, bucket string, title string, expires time.Duration) (*url.URL, string, error) {
	client, err := p.writable()
	if err != nil {
		return nil, "", err
	}

	object := uuid.New().String()
	{
		ext := filepath.Ext(title)
		if ext != "" {
			object += ext
		}
	}
	url, err := client.PresignedPutObject(ctx, bucket, object, expires)
	if err != nil {
		return nil, "", err
	}
	return url, object, nil
}

func (p *Client) PermanentUrl(ctx context.Context, bucket string, object string, title string, content_type *string) (*url.URL, error) {
	client, err := p.exists(ctx, bucket, object)
	if err != nil {
		return nil, err
	}
	params := make(url.Values)
	if content_type == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))
	} else {
		params.Set("response-content-type", *content_type)
	}
	it := client.EndpointURL().JoinPath(fmt.Sprintf("/%s/%s", bucket, object))
	it.RawQuery = params.Encode()
	return it, nil
}

func (p *Client) PresignedUrl(ctx context.Context, bucket string, object string, title string, content_type *string, expires time.Duration) (*url.URL, error) {
	client, err := p.exists(ctx, bucket, object)
	if err != nil {
		return nil, err
	}
	params := make(url.Values)
	if content_type == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))
	} else {
		params.Set("response-content-type", *content_type)
	}
	return client.PresignedGetObject(ctx, bucket, object, expires, params)
}

func (p *Client) writable() (*minio_.Client, error) {
	for _, it := range p.nodes {
		if !it.readonly {
			return it.client, nil
		}
	}
	return nil, errors.New("no writable minio node")
}

func (p *Client) exists(ctx context.Context, bucket string, object string) (*minio_.Client, error) {
	for _, node := range p.nodes {
		if _, err := node.client.StatObject(ctx, bucket, object, minio_.StatObjectOptions{}); err == nil {
			return node.client, nil
		}
	}
	return nil, errors.New("couldn't found the object")
}
