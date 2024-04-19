package env

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"github.com/minio/minio-go/v7/pkg/lifecycle"

	pb "github.com/saturn-xiv/palm/lilac/s3/v2"
)

type Minio struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	UseSSL    bool   `toml:"use-ssl"`
}

func (p *Minio) Open() (*minio.Client, error) {
	slog.Info(fmt.Sprintf("open minio %s", p.Endpoint))
	cli, err := minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessKey, p.SecretKey, ""),
		Secure: p.UseSSL,
	})
	if err != nil {
		return nil, err
	}
	{
		buckets, err := cli.ListBuckets(context.Background())
		if err != nil {
			return nil, err
		}
		var names []string
		for _, it := range buckets {
			names = append(names, it.Name)
		}
		slog.Debug(fmt.Sprintf("found buckets: %s", strings.Join(names, ",")))

	}
	return cli, nil
}

func CreateBucket(ctx context.Context, client *minio.Client, namespace string, req *pb.S3CreateBucketRequest) (string, error) {
	bucket, err := req.BucketName(namespace)
	if err != nil {
		return "", err
	}
	found, err := client.BucketExists(ctx, bucket)
	if err != nil {
		return "", err
	}
	if !found {
		slog.Info(fmt.Sprintf("create bucket %s", bucket))

		if err = client.MakeBucket(ctx, bucket, minio.MakeBucketOptions{}); err != nil {
			return "", err
		}
		if req.Public {
			slog.Info(fmt.Sprintf("set bucket %s to public", bucket))
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
			slog.Debug(policy)
			if err = client.SetBucketPolicy(ctx, bucket, policy); err != nil {
				return "", err
			}
		}

		if req.ExpirationDays != nil {
			slog.Info(fmt.Sprintf("set bucket %s expiration-days %d", bucket, *req.ExpirationDays))
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

			if err = client.SetBucketLifecycle(ctx, bucket, config); err != nil {
				return "", err
			}
		}
	}
	return bucket, nil
}
