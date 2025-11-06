package app

import (
	"fmt"
	"log/slog"
	"net"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	healthgrpc "google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn_xiv/palm/daisy/cache"
	"github.com/saturn_xiv/palm/daisy/crypto"
	crypto_v2 "github.com/saturn_xiv/palm/daisy/crypto/v2"
	"github.com/saturn_xiv/palm/daisy/queue"
	"github.com/saturn_xiv/palm/daisy/rbac"
	rbac_v2 "github.com/saturn_xiv/palm/daisy/rbac/v2"
	"github.com/saturn_xiv/palm/daisy/s3"
	s3_v2 "github.com/saturn_xiv/palm/daisy/s3/v2"
)

type RpcServerConfig struct {
	RabbitMQ *queue.RabbitMQ     `toml:"rabbitmq"`
	Minio    *s3.Config          `toml:"minio"`
	Database *Database           `toml:"database"`
	Redis    *cache.RedisCluster `toml:"redis"`
}

func LaunchRpcServer(config_file string, port uint16, debug bool) error {
	if debug {
		slog.SetLogLoggerLevel(slog.LevelDebug)
	} else {
		slog.SetLogLoggerLevel(slog.LevelInfo)
	}

	slog.Debug("load configuration from", "file", config_file)
	var config RpcServerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	aead, err := crypto.NewAead("aead.bin")
	if err != nil {
		return err
	}
	hmac, err := crypto.NewHmac("hmac.bin")
	if err != nil {
		return err
	}
	jwt, err := crypto.NewJwt("jwt.bin")
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	enforcer, err := rbac.NewEnforcer(db, config.Redis.Addresses(), config.Redis.Namespace)
	if err != nil {
		return err
	}
	s3_client, err := config.Minio.Open()
	if err != nil {
		return err
	}
	listen, err := net.Listen("tcp", fmt.Sprintf("0.0.0.0:%d", port))
	if err != nil {
		return err
	}

	server := grpc.NewServer()
	health_server := health.NewServer()
	healthgrpc.RegisterHealthServer(server, health_server)
	crypto_v2.RegisterAeadServer(server, crypto.NewAeadServer(aead))
	crypto_v2.RegisterHMacServer(server, crypto.NewHmacServer(hmac))
	crypto_v2.RegisterJwtServer(server, crypto.NewJwtServer(jwt))
	s3_v2.RegisterS3Server(server, s3.NewServer(s3_client))
	rbac_v2.RegisterEnforcerServer(server, rbac.NewServer(enforcer))

	slog.Info("gRPC server listening at", "address", listen.Addr())
	return server.Serve(listen)
}
