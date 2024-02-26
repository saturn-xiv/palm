package cmd

import (
	"fmt"
	"net"

	"github.com/BurntSushi/toml"
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/lilac/casbin"
	casbin_pb "github.com/saturn-xiv/palm/lilac/casbin/v2"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/minio"
	minio_pb "github.com/saturn-xiv/palm/lilac/minio/v2"
	"github.com/saturn-xiv/palm/lilac/tink"
	tink_pb "github.com/saturn-xiv/palm/lilac/tink/v2"
)

func launch_rpc_server(port int, config_file string, keys_dir string) error {
	log.Debugf("load configuration from %s", config_file)
	var config env.Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	enforcer, err := config.OpenCasbinEnforcer()
	if err != nil {
		return err
	}

	aes, hmac, jwt, err := crypto.Open(keys_dir)
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

	casbin_pb.RegisterPolicyServer(server, casbin.NewPolicyService(enforcer))

	minio_pb.RegisterS3Server(server, minio.NewS3Service())

	tink_pb.RegisterHmacServer(server, tink.NewHmacService(hmac))
	tink_pb.RegisterAesServer(server, tink.NewAesService(aes))
	tink_pb.RegisterJwtServer(server, tink.NewJwtService(jwt))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
