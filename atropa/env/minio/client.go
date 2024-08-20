package minio

import (
	"context"
	"errors"
	"log/slog"
	"net/url"
	"time"

	minio_ "github.com/minio/minio-go/v7"
)

type Client struct {
	namespace string
	client    *minio_.Client
}

func (p *Client) CreateBucket(ctx context.Context, name string, public bool, expiration_days int) (string, error) {
	it := bucket{
		namespace:       p.namespace,
		name:            name,
		public:          public,
		expiration_days: expiration_days,
	}
	name, err := it.code()
	if err != nil {
		return "", err
	}
	if err := create_bucket(ctx, p.client, name, public, expiration_days); err != nil {
		return "", err
	}
	return name, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *Client) Upload(ctx context.Context, bucket string, title string, ttl time.Duration) (*url.URL, string, error) {
	url, object, err := object_upload_via_browser(ctx, p.client, bucket, title, ttl)
	if err != nil {
		return nil, "", err
	}
	return url, object, nil
}

func (p *Client) PermanentUrl(ctx context.Context, bucket string, object string, title string, content_type *string) (*url.URL, error) {
	{
		it, err := bucket_from_code(bucket)
		if err != nil {
			return nil, err
		}
		if !it.public {
			return nil, errors.New("bucket is private")
		}
	}

	_, err := object_status(ctx, p.client, bucket, object)
	if err != nil {
		return nil, err
	}
	slog.Debug("found", slog.String("node", p.client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
	return object_permanent_url(p.client, bucket, object, title, content_type)
}

func (p *Client) PresignedUrl(ctx context.Context, bucket string, object string, title string, content_type *string, ttl time.Duration) (*url.URL, error) {
	{
		it, err := bucket_from_code(bucket)
		if err != nil {
			return nil, err
		}
		if it.public {
			return nil, errors.New("bucket is public")
		}
	}
	// _, err := object_status(ctx, p.client, bucket, object)
	// if err != nil {
	// 	return nil, err
	// }
	// slog.Debug("found", slog.String("node", p.client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
	return object_presigned_url(ctx, p.client, bucket, object, title, content_type, ttl)
}

func (p *Client) RemoveObject(ctx context.Context, bucket string, object string) error {
	return remove_object(ctx, p.client, bucket, object)
}
