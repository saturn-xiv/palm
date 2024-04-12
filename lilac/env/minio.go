package env

import (
	"context"
	"strings"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	log "github.com/sirupsen/logrus"
)

type Minio struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	UseSSL    bool   `toml:"use-ssl"`
}

func (p *Minio) Open() (*minio.Client, error) {
	log.Infof("open minio %s", p.Endpoint)
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
		log.Debugf("found buckets: %s", strings.Join(names, ","))

	}
	return cli, nil
}
