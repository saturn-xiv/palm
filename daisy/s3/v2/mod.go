package v2

import (
	"bytes"
	"context"
	_ "embed"
	"fmt"
	"log/slog"
	"net/url"
	"path/filepath"
	"strings"
	"text/template"
	"time"

	"github.com/google/uuid"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/lifecycle"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
)

//go:embed anonymous-read.json
var anonymous_read_json string

func (p *StatObjectRequest) Execute(ctx context.Context, client *minio.Client) (*StatObjectResponse, error) {
	slog.Debug("stat", "bucket", p.Bucket, "object", p.Object)
	stat, err := client.StatObject(ctx, p.Bucket, p.Object, minio.StatObjectOptions{})
	if err != nil {
		return nil, err
	}
	return &StatObjectResponse{
		ContentType:  stat.ContentType,
		Size:         stat.Size,
		Expires:      timestamppb.New(stat.Expires),
		LastModified: timestamppb.New(stat.LastModified),
		VersionId:    stat.VersionID,
	}, nil
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html
func ObjectName(title string) string {
	return uuid.New().String() + filepath.Ext(title)
}

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
func BucketName(namespace string, public bool, expire_after_days *uint32) string {
	var days uint32
	if expire_after_days != nil {
		days = *expire_after_days
	} else {
		days = 0
	}
	var pub string
	if public {
		pub = "t"
	} else {
		pub = "f"
	}
	now := time.Now()
	return fmt.Sprintf("%s.%s%s%d", strings.ToLower(strings.TrimSpace(namespace)), now.Format("200601"), pub, days)
}

func (p *MakeBucketRequest) Execute(ctx context.Context, client *minio.Client) error {
	slog.Info("create bucket", "name", p.Name)
	err := client.MakeBucket(ctx, p.Name, minio.MakeBucketOptions{})
	if err != nil {
		return err
	}
	if p.Public {
		slog.Info("set anonymous read access", "bucket", p.Name)
		tpl, err := template.New("anonymous-read").Parse(anonymous_read_json)
		if err != nil {
			return err
		}
		var buf bytes.Buffer
		if err = tpl.Execute(&buf, map[string]interface{}{"bucket": p.Name}); err != nil {
			return err
		}
		policy := buf.String()
		slog.Debug("set policy", "rule", policy)
		if err = client.SetBucketPolicy(ctx, p.Name, policy); err != nil {
			return err
		}
	}
	if p.ExpireAfterDays != nil {
		config := lifecycle.NewConfiguration()
		config.Rules = []lifecycle.Rule{
			{
				ID:     fmt.Sprintf("expire-after-%d-days", *p.ExpireAfterDays),
				Status: "Enabled",
				Expiration: lifecycle.Expiration{
					Days: lifecycle.ExpirationDays(*p.ExpireAfterDays),
				},
			},
		}

		if err = client.SetBucketLifecycle(ctx, p.Name, config); err != nil {
			return err
		}
	}

	return nil
}

func (p *RemoveObjectRequest) Execute(ctx context.Context, client *minio.Client) error {
	return client.RemoveObject(ctx, p.Bucket, p.Object, minio.RemoveObjectOptions{ForceDelete: true})
}

func (p *PresignedPutObjectRequest) Execute(ctx context.Context, client *minio.Client) (*url.URL, error) {
	return client.PresignedPutObject(ctx, p.Bucket, p.Object, p.Ttl.AsDuration())
}

func (p *GetObjectRequest) Execute(client *minio.Client) *url.URL {
	return client.EndpointURL().JoinPath(p.Bucket, p.Object)
}

func (p *PresignedGetObjectRequest) Execute(ctx context.Context, client *minio.Client) (*url.URL, error) {
	params := make(url.Values)
	if p.Title != nil {
		params.Set("response-content-disposition", fmt.Sprintf("attachment; filename=\"%s\"", *p.Title))
	}
	return client.PresignedGetObject(ctx, p.Bucket, p.Object, p.Ttl.AsDuration(), params)
}
