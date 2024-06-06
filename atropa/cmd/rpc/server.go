package rpc

import (
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/BurntSushi/toml"
	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"google.golang.org/grpc/reflection"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/services"
	pb "github.com/saturn-xiv/palm/atropa/services/v2"
)

func Launch(port uint16, config_file string, keys_dir string, version string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	enforcer, err := env.OpenCasbinEnforcer(config.Namespace, db, config.Redis.Options().Addrs)
	if err != nil {
		return err
	}

	aes, mac, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}

	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

	address := fmt.Sprintf("0.0.0.0:%d", port)
	network := "tcp"
	slog.Info(fmt.Sprintf("start gRPC on %s://%s", network, address))
	socket, err := net.Listen(network, address)
	if err != nil {
		return err
	}
	var options []grpc.ServerOption

	server := grpc.NewServer(options...)
	if err = mount(server, config.Namespace, aes, mac, jwt, enforcer, s3); err != nil {
		return err
	}
	reflection.Register(server)

	go func() {
		if err := server.Serve(socket); err != nil {
			slog.Error(err.Error())
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	slog.Warn("shutting down gRPC server...")

	server.GracefulStop()
	slog.Info("server exiting")
	return nil
}

func mount(server *grpc.Server, namespace string,
	aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt,
	enforcer *casbin.Enforcer,
	s3 *minio.Client,
) error {
	pb.RegisterAesServer(server, services.NewAesService(aes))
	pb.RegisterJwtServer(server, services.NewJwtService(jwt))
	pb.RegisterHMacServer(server, services.NewHmacService(mac))
	pb.RegisterPolicyServer(server, services.NewPolicyService(enforcer))
	pb.RegisterS3Server(server, services.NewS3Service(namespace, s3))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return nil
}
