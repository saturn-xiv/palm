package rpc

import (
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
)

type Config struct {
	Namespace string          `toml:"namespace"`
	Redis     redis.Cluster   `toml:"redis"`
	Database  env.Database    `toml:"database"`
	Minio     env.Minio       `toml:"minio"`
	RabbitMq  rabbitmq.Config `toml:"rabbitmq"`
}
