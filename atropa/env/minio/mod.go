package minio

import (
	"bytes"
	"context"
	"encoding/base32"
	"encoding/gob"
	"fmt"
	"log/slog"
	"strings"

	minio_ "github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

type Node struct {
	Endpoint  string `toml:"endpoint"`
	AccessKey string `toml:"access-key"`
	SecretKey string `toml:"secret-key"`
	Secure    bool   `toml:"secure"`
	Readonly  bool   `toml:"readonly"`
}

type Config struct {
	Namespace string `toml:"namespace"`
	Nodes     []Node `toml:"nodes"`
}

func (p *Node) open() (*node, error) {
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
	return &node{client: client, readonly: p.Readonly}, nil
}

func (p *Config) Open() (*Client, error) {
	var nodes []*node
	for _, node := range p.Nodes {
		it, err := node.open()
		if err != nil {
			return nil, err
		}
		nodes = append(nodes, it)
	}
	return &Client{nodes: nodes, namespace: p.Namespace}, nil
}

type bucket struct {
	namespace       string
	name            string
	public          bool
	expiration_days int
}

func (p *bucket) code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	code := base32.HexEncoding.WithPadding(base32.NoPadding).EncodeToString(buf.Bytes())
	return strings.ToLower(code), nil
}
