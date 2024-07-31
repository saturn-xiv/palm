package minio

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net/url"
	"time"

	minio_ "github.com/minio/minio-go/v7"
	"github.com/mroth/weightedrand/v2"
)

type Cluster struct {
	namespace string
	chooser   *weightedrand.Chooser[*minio_.Client, uint8]
	clients   []*minio_.Client
}

func (p *Cluster) CreateBucket(ctx context.Context, name string, public bool, expiration_days int) (string, error) {
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
	for _, client := range p.clients {
		if err := create_bucket(ctx, client, name, public, expiration_days); err != nil {
			return "", err
		}
	}
	return name, nil
}

// https://min.io/docs/minio/linux/integrations/presigned-put-upload-via-browser.html
func (p *Cluster) Upload(ctx context.Context, bucket string, title string, ttl time.Duration) (*url.URL, string, error) {
	client := p.chooser.Pick()
	url, object, err := object_upload_via_browser(ctx, client, bucket, title, ttl)
	if err != nil {
		return nil, "", err
	}
	return url, object, nil
}

func (p *Cluster) PermanentUrl(ctx context.Context, bucket string, object string, title string, content_type *string) (*url.URL, error) {
	{
		it, err := bucket_from_code(bucket)
		if err != nil {
			return nil, err
		}
		if !it.public {
			return nil, errors.New("bucket is private")
		}
	}
	for _, client := range p.clients {
		if _, err := object_status(ctx, client, bucket, object); err == nil {
			slog.Debug("found", slog.String("node", client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
			return object_permanent_url(client, bucket, object, title, content_type)
		}
	}
	return nil, fmt.Errorf("couldn't found %s/%s on any nodes", bucket, object)
}

func (p *Cluster) PresignedUrl(ctx context.Context, bucket string, object string, title string, content_type *string, ttl time.Duration) (*url.URL, error) {
	{
		it, err := bucket_from_code(bucket)
		if err != nil {
			return nil, err
		}
		if it.public {
			return nil, errors.New("bucket is public")
		}
	}
	for _, client := range p.clients {
		if _, err := object_status(ctx, client, bucket, object); err == nil {
			slog.Debug("found", slog.String("node", client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
			return object_presigned_url(ctx, client, bucket, object, title, content_type, ttl)
		}
	}
	return nil, fmt.Errorf("couldn't found %s/%s on any nodes", bucket, object)
}

func (p *Cluster) RemoveObject(ctx context.Context, bucket string, object string) error {
	for _, client := range p.clients {
		if _, err := object_status(ctx, client, bucket, object); err == nil {
			slog.Debug("found", slog.String("node", client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
			return remove_object(ctx, client, bucket, object)
		}
	}
	return fmt.Errorf("couldn't found %s/%s on any nodes", bucket, object)
}
