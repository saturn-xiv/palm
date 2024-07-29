package minio

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	minio_ "github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"github.com/mroth/weightedrand/v2"
)

type Node struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	Secure    bool   `toml:"secure"`
	Readonly  bool   `toml:"readonly"`
	Weight    uint8  `toml:"weight"`
}

type Config struct {
	Namespace string `toml:"namespace"`
	Nodes     []Node `toml:"nodes"`
}

func (p *Node) open() (*Client, error) {
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
	return &Client{client: client, Readonly: p.Readonly}, nil
}

func (p *Config) Open() (*Cluster, error) {
	var clients []*Client
	var choices []weightedrand.Choice[*Client, uint8]
	for _, it := range p.Nodes {
		cli, err := it.open()
		if err != nil {
			return nil, err
		}
		if !it.Readonly {
			choices = append(choices, weightedrand.NewChoice(cli, it.Weight))
		}
		clients = append(clients, cli)
	}
	chooser, err := weightedrand.NewChooser(choices...)
	if err != nil {
		return nil, err
	}
	return &Cluster{Namespace: p.Namespace, Chooser: chooser, Clients: clients}, nil
}
