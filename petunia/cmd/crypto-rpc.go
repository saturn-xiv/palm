package cmd

import (
	"fmt"
	"net"

	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/petunia/crypto"
	crypto_pb "github.com/saturn-xiv/palm/petunia/crypto/v2"
)

func launch_crypto_rpc_server(port int, app_id string) error {

	aes, hmac, jwt, err := crypto.Open(app_id)
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
	crypto_pb.RegisterHmacServer(server, crypto.NewHmacService(hmac))
	crypto_pb.RegisterAesServer(server, crypto.NewAesService(aes))
	crypto_pb.RegisterJwtServer(server, crypto.NewJwtService(jwt))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
