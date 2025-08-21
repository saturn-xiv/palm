package redis

import (
	"context"
	"fmt"
	"log/slog"
	"time"

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

func (p *Cluster) Open(ctx context.Context) (*Client, error) {
	options := p.Options()
	db := redis.NewClusterClient(&options)
	if err := db.ForEachShard(ctx, func(ctx context.Context, shard *redis.Client) error {
		slog.Debug("test redis cluster", slog.String("node", shard.Options().Addr))
		return shard.Ping(ctx).Err()
	}); err != nil {
		return nil, err
	}
	return &Client{db: db, namespace: p.Namespace}, nil
}

type Node struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}

type Client struct {
	db        *redis.ClusterClient
	namespace string
}

func (p *Client) Set(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	return p.db.Set(ctx, p.key(key), value, ttl).Err()
}
func (p *Client) Get(ctx context.Context, key string) ([]byte, error) {
	return p.db.Get(ctx, key).Bytes()
}
func (p *Client) GetF(ctx context.Context, key string, fn func() ([]byte, error), ttl time.Duration) ([]byte, error) {
	val, err := p.Get(ctx, key)
	if err == nil {
		return val, nil
	}
	if err != redis.Nil {
		return nil, err
	}
	slog.Debug("couldn't found, try to set it", slog.String("key", key))
	val, err = fn()
	if err != nil {
		return nil, err
	}
	if err = p.Set(ctx, key, val, ttl); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *Client) key(s string) string {
	return fmt.Sprintf("%s://%s", p.namespace, s)
}
