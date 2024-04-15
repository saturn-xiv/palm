package redis

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	"github.com/redis/go-redis/v9"
)

type Cluster struct {
	Nodes []Node `toml:"nodes"`
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

func (p *Cluster) Open(namespace string) (*Client, error) {
	slog.Info(fmt.Sprintf("open redis %s", strings.Join(p.Addrs(), ",")))
	options := p.Options()
	db := redis.NewClusterClient(&options)
	{
		ctx := context.Background()
		status := db.ClusterNodes(ctx)
		if err := status.Err(); err != nil {
			return nil, err
		}
		slog.Debug(fmt.Sprintf("redis nodes:\n%s", status.Val()))

		if err := db.ForEachShard(ctx, func(ctx context.Context, shard *redis.Client) error {
			status := shard.Ping(ctx)
			return status.Err()
		}); err != nil {
			return nil, err
		}
	}
	return &Client{
		namespace: namespace,
		db:        db,
	}, nil
}

type Node struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}
