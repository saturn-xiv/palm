package env

import (
	"fmt"
	"log/slog"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Jasmine struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}

func (p *Jasmine) Open() (*minio.Client, error) {
	slog.Debug(fmt.Sprintf("open jasmine tcp://%s:%d", p.Host, p.Port))
	return minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessKey, p.SecretKey, ""),
		Secure: p.UseSSL,
	})
}
