package rpc

import (
	"fmt"
	"log/slog"
	"net"
	"reflect"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	grpc_health "google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/services/s3"
	s3_v2 "github.com/saturn-xiv/palm/jasmine/services/s3/v2"
)

func Launch(port uint16, config_file string, version string) error {
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
	s3_v2.RegisterS3Server(server, s3.NewS3Server(db, jwt, enf, minio_client))
	grpc_health.RegisterHealthServer(server, health.NewServer())

	listen, err := net.Listen("tcp", fmt.Sprintf("0.0.0.0:%d", port))
	if err != nil {
		return err
	}
	slog.Info("server listening at", slog.String("address", listen.Addr().String()))
	if err := server.Serve(listen); err != nil {
		return err
	}

	return nil
}

func serviceName(it interface{}) string {
	return fmt.Sprintf("%s/%s", reflect.TypeOf(it).Elem().PkgPath(), reflect.TypeOf(it).Elem().Name())
}
