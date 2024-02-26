package redis

import (
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

type Node struct {
	Host string `toml:"host"`
	Port uint16 `toml:"port"`
}
