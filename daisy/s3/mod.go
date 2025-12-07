package s3

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Config struct {
	Endpoint  string `toml:"endpoint"`
	AccessId  string `toml:"access-id"`
	SecretKey string `toml:"secret-key"`
}

func (p *Config) New(file string) (*Config, error) {
	buf, err := os.ReadFile(file)
	if err != nil {
		return nil, err
	}
	var it Config
	if err = json.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Config) Open() (*minio.Client, error) {
	slog.Info("open minio", "endpoint", p.Endpoint)
	cli, err := minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessId, p.SecretKey, ""),
		Secure: true,
	})
	if err != nil {
		return nil, err
	}
	{
		ctx := context.Background()
		res, err := cli.ListBuckets(ctx)
		if err != nil {
			return nil, err
		}
		slog.Debug("list buckets", "count", len(res))
	}
	return cli, nil
}
