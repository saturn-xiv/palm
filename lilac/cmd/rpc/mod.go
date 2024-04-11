package rpc

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/redis"
)

type Config struct {
	Redis      redis.Cluster  `toml:"redis"`
	PostgreSql env.PostgreSql `toml:"postgresql,omitempty"`
	MySql      env.MySql      `toml:"mysql,omitempty"`
	SqlServer  env.SqlServer  `toml:"sqlserver,omitempty"`
	Minio      env.Minio      `toml:"minio"`
}

func (p *Config) OpenDb() (*gorm.DB, error) {
	if len(p.PostgreSql.DbName) > 0 {
		return p.PostgreSql.Open()
	}
	if len(p.MySql.DbName) > 0 {
		return p.MySql.Open()
	}
	if len(p.SqlServer.DbName) > 0 {
		return p.SqlServer.Open()
	}
	it := env.Sqlite3{File: "db"}
	return it.Open()
}
