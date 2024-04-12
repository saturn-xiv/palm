package rpc

import (
	"fmt"
	"net"

	"github.com/BurntSushi/toml"
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/services"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

func Launch(port int, config_file string, keys_dir string) error {
	log.Debugf("load configuration from %s", config_file)
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.OpenDb()
	if err != nil {
		return err
	}
	enforcer, err := env.OpenCasbinEnforcer(db, config.Redis.Options().Addrs, config.Redis.Namespace)
	if err != nil {
		return err
	}

	aes, mac, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}

	minio, err := config.Minio.Open()
	if err != nil {
		return err
	}

	network := "tcp"
	address := fmt.Sprintf("0.0.0.0:%d", port)
	log.Infof("start gRPC on %s://%s", network, address)
	socket, err := net.Listen(network, address)
	if err != nil {
		log.Fatalln(err)
	}
	var opts []grpc.ServerOption

	server := grpc.NewServer(opts...)

	pb.RegisterUserServer(server, services.NewUserService(aes, mac, jwt, enforcer))
	pb.RegisterPolicyServer(server, services.NewPolicyService(jwt))
	pb.RegisterSmsServer(server, services.NewSmsService(jwt))
	pb.RegisterEmailServer(server, services.NewEmailService(jwt))
	pb.RegisterS3Server(server, services.NewS3Service(minio, jwt))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
