package env

import (
	"crypto/tls"
	"crypto/x509"
	"errors"
	"fmt"
	"os"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

// https://github.com/grpc/grpc/blob/master/doc/naming.md
type GrpcClient struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
	Tls  Tls    `toml:"tls"`
}

func (p *GrpcClient) Open() (*grpc.ClientConn, error) {
	tls, err := p.Tls.Load(false)
	if err != nil {
		return nil, err
	}
	return grpc.NewClient(p.Address(), grpc.WithTransportCredentials(credentials.NewTLS(tls)))
}

func (p *GrpcClient) Address() string {
	return fmt.Sprintf("%s:%d", p.Host, p.Port)
}

type Tls struct {
	CaFile   string `toml:"ca-file"`
	CertFile string `toml:"cert-file"`
	KeyFile  string `toml:"key-file"`
}

func (p *Tls) Load(server bool) (*tls.Config, error) {
	root_ca := x509.NewCertPool()
	{
		buf, err := os.ReadFile(p.CaFile)
		if err != nil {
			return nil, err
		}
		if !root_ca.AppendCertsFromPEM(buf) {
			return nil, errors.New("failed to add server CA's certificate")
		}
	}
	cert, err := tls.LoadX509KeyPair(p.CertFile, p.KeyFile)
	if err != nil {
		return nil, err
	}
	config := tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs:      root_ca,
	}
	if server {
		config.ClientAuth = tls.RequestClientCert
	}
	return &config, nil
}
