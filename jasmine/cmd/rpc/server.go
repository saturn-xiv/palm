package rpc

import (
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	grpc_health "google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/services/casbin"
	casbin_v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/s3"
	s3_v2 "github.com/saturn-xiv/palm/jasmine/services/s3/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func Launch(port uint16, config_file string, version string) error {
	web.EnsureStopped()
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, _, jwt, err := crypto.Open(config.SecretsStore)
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	enf, err := env.OpenCasbinEnforcer(config.Redis.Namespace, db, config.Redis.Options().Addrs)
	if err != nil {
		return err
	}
	minio_client, err := config.Minio.Open()
	if err != nil {
		return err
	}

	server := grpc.NewServer()
	casbin_v2.RegisterPolicyServer(server, casbin.NewPolicyServer(db, jwt, enf))
	s3_v2.RegisterS3Server(server, s3.NewS3Server(db, jwt, enf, minio_client))
	grpc_health.RegisterHealthServer(server, health.NewServer())

	listen, err := net.Listen("tcp", fmt.Sprintf("0.0.0.0:%d", port))
	if err != nil {
		return err
	}

	start(server, listen)
	return nil
}

// https://grpc.io/docs/guides/server-graceful-stop/
func start(server *grpc.Server, listen net.Listener) {
	slog.Info("server listening at", slog.String("address", listen.Addr().String()))
	go func() {
		if err := server.Serve(listen); err != nil {
			slog.Error(err.Error())
		}
	}()
	c := make(chan os.Signal, 1)

	signal.Notify(c, os.Interrupt)

	<-c

	server.GracefulStop()

	slog.Warn("server stopped gracefully")
	os.Exit(0)
}
