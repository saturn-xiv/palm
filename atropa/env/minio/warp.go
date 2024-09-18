package minio

import (
	"bytes"
	"context"
	"encoding/base32"
	"encoding/gob"
	"fmt"
	"log/slog"
	"net/url"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	minio_ "github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
)

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func create_bucket(ctx context.Context, client *minio_.Client, name string, public bool, expiration_days int) error {
	found, err := client.BucketExists(ctx, name)
	if err != nil {
		return err
	}
	if !found {
		slog.Info("create bucket", slog.String("name", name), slog.String("node", client.EndpointURL().Host))
		if err = client.MakeBucket(ctx, name, minio_.MakeBucketOptions{}); err != nil {
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
			if err = client.SetBucketPolicy(ctx, name, policy); err != nil {
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

			if err = client.SetBucketLifecycle(ctx, name, config); err != nil {
				return err
			}
		}
	}
	return nil
}

func object_upload_via_file(ctx context.Context, client *minio_.Client, file string, content_type string, bucket string, object string) error {
	slog.Debug("upload", slog.String("endpoint", client.EndpointURL().String()), slog.String("file", file), slog.String("bucket", bucket), slog.String("object", object))
	info, err := client.FPutObject(ctx, bucket, object, file, minio_.PutObjectOptions{ContentType: content_type})
	if err != nil {
		return err
	}
	slog.Info("upload done", slog.Int64("size", info.Size))
	return nil
}

func presigned_put_object(ctx context.Context, client *minio_.Client, bucket string, title string, expires time.Duration) (*url.URL, string, error) {
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

func remove_bucket(ctx context.Context, client *minio_.Client, name string) error {
	slog.Warn("delete", slog.String("node", client.EndpointURL().Host), slog.String("bucket", name))
	return client.RemoveBucket(ctx, name)
}
func remove_object(ctx context.Context, client *minio_.Client, bucket string, object string) error {
	slog.Warn("delete", slog.String("node", client.EndpointURL().Host), slog.String("bucket", bucket), slog.String("object", object))
	return client.RemoveObject(ctx, bucket, object, minio_.RemoveObjectOptions{})
}
func object_permanent_url(client *minio_.Client, bucket string, object string, title string, content_type *string) (*url.URL, error) {
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

func object_presigned_url(ctx context.Context, client *minio_.Client, bucket string, object string, title string, content_type *string, expires time.Duration) (*url.URL, error) {
	params := make(url.Values)
	if content_type == nil {
		params.Set("response-content-disposition", fmt.Sprintf(`attachment; filename="%s"`, title))
	} else {
		params.Set("response-content-type", *content_type)
	}
	return client.PresignedGetObject(ctx, bucket, object, expires, params)
}

func object_status(ctx context.Context, client *minio_.Client, bucket string, object string) (*minio_.ObjectInfo, error) {
	status, err := client.StatObject(ctx, bucket, object, minio_.StatObjectOptions{})
	if err != nil {
		return nil, err
	}
	return &status, nil
}

type bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days int
}

func (p *bucket) code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	code := base32.HexEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes())
	return strings.ToLower(code), nil
}

func bucket_from_code(code string) (*bucket, error) {
	bin, err := base32.HexEncoding.WithPadding(base32.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)

	var it bucket
	if err := dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}
