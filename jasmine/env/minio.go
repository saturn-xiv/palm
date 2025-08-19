package env

import (
	"context"
	"log/slog"
	"strings"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Minio struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	Secure    bool   `toml:"secure"`
}

func (p *Minio) Open() (*minio.Client, error) {
	slog.Info("open minio", slog.String("endpoint", p.Endpoint))
	cli, err := minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessKey, p.SecretKey, ""),
		Secure: p.Secure,
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
		slog.Debug("found buckets", slog.String("buckets", strings.Join(names, ",")))

	}
	return cli, nil
}
