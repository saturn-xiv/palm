package cmd

import (
	"fmt"
	"net"

	"github.com/BurntSushi/toml"
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/petunia/rbac"
	rbac_pb "github.com/saturn-xiv/palm/petunia/rbac/v2"
)

func launch_rbac_rpc_server(port int, config_file string) error {
	log.Debugf("load configuration from %s", config_file)
	var config rbac.Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	enforcer, err := config.OpenCasbinEnforcer()
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
	rbac_pb.RegisterPolicyServer(server, rbac.NewPolicyService(enforcer))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
