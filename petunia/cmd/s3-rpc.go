package cmd

import (
	"encoding/json"
	"fmt"
	"net"
	"os"

	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/petunia/env"
	"github.com/saturn-xiv/palm/petunia/s3"
	s3_pb "github.com/saturn-xiv/palm/petunia/s3/v2"
)

func launch_rpc_server(port int, config_file string) error {
	log.Debugf("load configuration from %s", config_file)
	var config env.Minio
	{
		buf, err := os.ReadFile(config_file)
		if err != nil {
			return err
		}
		if err = json.Unmarshal(buf, &config); err != nil {
			return err
		}
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
	s3_pb.RegisterS3Server(server, s3.NewS3Service())

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
