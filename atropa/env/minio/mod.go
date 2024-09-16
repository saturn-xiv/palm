package minio

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	minio_ "github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Config struct {
	Namespace string `toml:"namespace"`
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	Secure    bool   `toml:"secure"`
}

func (p *Config) open() (*minio_.Client, error) {
	slog.Info(fmt.Sprintf("open minio %s", p.Endpoint))
	client, err := minio_.New(p.Endpoint, &minio_.Options{
		Creds:  credentials.NewStaticV4(p.AccessKey, p.SecretKey, ""),
		Secure: p.Secure,
	})
	if err != nil {
		return nil, err
	}
	{
		buckets, err := client.ListBuckets(context.Background())
		if err != nil {
			return nil, err
		}
		var names []string
		for _, it := range buckets {
			names = append(names, it.Name)
		}
		slog.Debug(fmt.Sprintf("found buckets: %s", strings.Join(names, ",")))

	}
	return client, nil
}

func (p *Config) Open() (*Client, error) {
	client, err := p.open()
	if err != nil {
		return nil, err
	}
	return &Client{namespace: p.Namespace, client: client}, nil
}
