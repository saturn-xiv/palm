package web

import (
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/env/redis"
)

type Config struct {
	Theme     string          `toml:"theme"`
	Namespace string          `toml:"namespace"`
	Database  env.Database    `toml:"database"`
	Redis     redis.Cluster   `toml:"redis"`
	RabbitMQ  rabbitmq.Config `toml:"rabbitmq"`
	Minio     env.Minio       `toml:"minio"`
}
