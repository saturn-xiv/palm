package s3

import (
	"encoding/json"
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
	return minio.New(p.Endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(p.AccessId, p.SecretKey, ""),
		Secure: true,
	})
}
