package redis

import (
	"context"
	"fmt"

	"github.com/redis/go-redis/v9"
)

type Cluster struct {
	Nodes     []Node `toml:"nodes"`
	Namespace string `toml:"namespace"`
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

func (p *Cluster) Open(ctx context.Context) (*redis.ClusterClient, error) {
	options := p.Options()
	db := redis.NewClusterClient(&options)
	if err := db.ForEachShard(ctx, func(ctx context.Context, shard *redis.Client) error {
		return shard.Ping(ctx).Err()
	}); err != nil {
		return nil, err
	}
	return db, nil
}

type Node struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}
