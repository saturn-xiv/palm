package cache

import (
	"bytes"
	"context"
	"encoding/gob"
	"fmt"
	"log/slog"
	"time"

	"github.com/redis/go-redis/v9"
	"google.golang.org/protobuf/proto"
)

type RedisCluster struct {
	Nodes     []RedisNode `toml:"nodes"`
	PoolSize  uint        `toml:"pool-size"`
	Namespace string      `toml:"namespace"`
}

func (p *RedisCluster) Addresses() []string {
	var items []string
	for _, it := range p.Nodes {
		items = append(items, fmt.Sprintf("%s:%d", it.Host, it.Port))
	}
	return items
}

func (p *RedisCluster) Open(ctx context.Context) (*RedisClient, error) {
	addrs := p.Addresses()
	slog.Info("open redis", "address", addrs)
	db := redis.NewClusterClient(&redis.ClusterOptions{
		Addrs:         addrs,
		RouteRandomly: true,
	})

	if err := db.ForEachShard(ctx, func(ctx context.Context, shard *redis.Client) error {
		return shard.Ping(ctx).Err()
	}); err != nil {
		return nil, err
	}
	return &RedisClient{
		db:        db,
		namespace: p.Namespace,
	}, nil
}

type RedisNode struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}

type RedisClient struct {
	namespace string
	db        *redis.ClusterClient
}

func (p *RedisClient) SetProtobuf(ctx context.Context, key string, value proto.Message, ttl time.Duration) error {
	buf, err := proto.Marshal(value)
	if err != nil {
		return err
	}
	return p.Set(ctx, key, buf, ttl)
}
func (p *RedisClient) GetProtobuf(ctx context.Context, key string, value proto.Message) error {
	tmp, err := p.Get(ctx, key)
	if err != nil {
		return err
	}
	return proto.Unmarshal(tmp, value)
}
func (p *RedisClient) GetB(ctx context.Context, key string, value interface{}) error {
	tmp, err := p.Get(ctx, key)
	if err != nil {
		return err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	return dec.Decode(value)
}
func (p *RedisClient) SetB(ctx context.Context, key string, value interface{}, ttl time.Duration) error {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(value); err != nil {
		return err
	}
	return p.Set(ctx, key, buf.Bytes(), ttl)
}
func (p *RedisClient) Set(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	return p.db.SetEx(ctx, p.key(key), value, ttl).Err()
}

func (p *RedisClient) Get(ctx context.Context, key string) ([]byte, error) {
	return p.db.Get(ctx, p.key(key)).Bytes()
}

func (p *RedisClient) Fetch(ctx context.Context, key string, fn func() ([]byte, error), ttl time.Duration) ([]byte, error) {
	key = p.key(key)
	if val, err := p.db.Get(ctx, key).Bytes(); err == nil {
		return val, nil
	}
	val, err := fn()
	if err != nil {
		return nil, err
	}
	if err = p.db.SetEx(ctx, key, val, ttl).Err(); err != nil {
		return nil, err
	}
	return val, nil
}

func (p *RedisClient) key(key string) string {
	return fmt.Sprintf("%s://%s", p.namespace, key)
}
