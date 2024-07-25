package redis

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	"github.com/redis/go-redis/v9"
)

type Cluster struct {
	Namespace string `toml:"namespace"`
	Nodes     []Node `toml:"nodes"`
}

func (p *Cluster) Addrs() []string {
	items := make([]string, 0)
	for _, it := range p.Nodes {
		items = append(items, fmt.Sprintf("%s:%d", it.Host, it.Port))
	}
	return items
}

func (p *Cluster) Options() redis.ClusterOptions {
	return redis.ClusterOptions{Addrs: p.Addrs()}
}

func (p *Cluster) Open() (*Client, error) {
	slog.Info(fmt.Sprintf("open redis %s", strings.Join(p.Addrs(), ",")))
	options := p.Options()
	client := Client{
		namespace: p.Namespace,
		db:        redis.NewClusterClient(&options),
	}
	{
		ctx := context.Background()
		status, err := client.Heartbeat(ctx)
		if err != nil {
			return nil, err
		}
		slog.Debug(fmt.Sprintf("redis nodes:\n%s", strings.Join(status, "\n")))
	}
	return &client, nil
}

type Node struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}
