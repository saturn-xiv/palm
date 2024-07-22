package redis

import (
	"bytes"
	"context"
	"encoding/gob"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/redis/go-redis/v9"
)

type HandlerFunc func(value interface{}) error

type Client struct {
	db        *redis.ClusterClient
	namespace string
}

func (p *Client) Set(ctx context.Context, key string, value interface{}, ttl time.Duration) error {
	k := p.key(key)
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(value); err != nil {
		return err
	}
	status := p.db.Set(ctx, k, buf.Bytes(), ttl)
	return status.Err()
}

func (p *Client) Get_(ctx context.Context, key string, value interface{}) error {
	k := p.key(key)
	status := p.db.Get(ctx, k)
	if err := status.Err(); err != nil {
		return err
	}
	bin, err := status.Bytes()
	if err != nil {
		return err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	return dec.Decode(value)
}

func (p *Client) Get(ctx context.Context, key string, handler HandlerFunc, value interface{}, ttl time.Duration) error {
	k := p.key(key)
	status := p.db.Get(ctx, k)
	err := status.Err()
	if err != nil {
		slog.Warn(fmt.Sprintf("couldn't catch %s %s", k, err))
		if err = handler(value); err != nil {
			return err
		}
		var buf bytes.Buffer
		enc := gob.NewEncoder(&buf)
		if err = enc.Encode(value); err != nil {
			return err
		}
		status := p.db.Set(ctx, k, buf.Bytes(), ttl)
		return status.Err()
	}

	bin, err := status.Bytes()
	if err != nil {
		return err
	}
	buf := bytes.NewBuffer(bin)
	dec := gob.NewDecoder(buf)
	return dec.Decode(value)

}

func (p *Client) key(k string) string {
	return fmt.Sprintf("%s://%s", p.namespace, k)
}

func (p *Client) Heartbeat(ctx context.Context) ([]string, error) {
	if err := p.db.ForEachShard(ctx, func(ctx context.Context, shard *redis.Client) error {
		status := shard.Ping(ctx)
		return status.Err()

	}); err != nil {
		return nil, err
	}
	status := p.db.ClusterNodes(ctx)
	if err := status.Err(); err != nil {
		return nil, err
	}
	return strings.Split(status.Val(), "\n"), nil
}
