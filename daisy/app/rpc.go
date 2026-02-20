package app

import (
	"fmt"
	"log"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	healthgrpc "google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/cups"
	cups_v2 "github.com/saturn-xiv/palm/daisy/cups/v2"
	"github.com/saturn-xiv/palm/daisy/queue"
	"github.com/saturn-xiv/palm/daisy/rbac"
	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
	"github.com/saturn-xiv/palm/daisy/s3"
	s3_v2 "github.com/saturn-xiv/palm/daisy/s3/v2"
)

type RpcServerConfig struct {
	RabbitMQ *queue.RabbitMQ     `toml:"rabbitmq"`
	Minio    *s3.Config          `toml:"minio"`
	Database *Database           `toml:"database"`
	Redis    *cache.RedisCluster `toml:"redis"`
}

func LaunchRpcServer(config_file string, port uint16, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config RpcServerConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	if _, _, _, err := open_secrets(); err != nil {
		return err
	}
	db, err := config.Database.Open(debug)
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

	cups_v2.RegisterCupsServer(server, cups.NewServer())
	s3_v2.RegisterS3Server(server, s3.NewServer(s3_client))
	rbac_v2.RegisterEnforcerServer(server, rbac.NewServer(enforcer))

	signal_chan := make(chan os.Signal, 1)
	signal.Notify(signal_chan, os.Interrupt, syscall.SIGTERM, syscall.SIGINT)

	go func() {
		slog.Info("gRPC server listening at", "address", listen.Addr())
		if err := server.Serve(listen); err != nil {
			log.Fatalf("failed to serve: %v", err)
		}
	}()

	sig := <-signal_chan
	slog.Warn("received", "signal", sig)

	server.GracefulStop()
	slog.Info("gRPC server stopped")
	return nil
}
