package env

import (
	"fmt"
	"log/slog"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Minio struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	UseSSL    bool   `toml:"use-ssl"`
}

func (p *Minio) Open() (*minio.Client, error) {
	slog.Info(fmt.Sprintf("open minio %s", p.Endpoint))
	return minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessKey, p.SecretKey, ""),
		Secure: p.UseSSL,
	})
}
