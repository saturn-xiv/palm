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
	"github.com/saturn-xiv/palm/lilac/models"
	"github.com/saturn-xiv/palm/lilac/services"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

func Launch(port int, config_file string, keys_dir string) error {
	log.Debugf("load configuration from %s", config_file)
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	cache, err := config.Redis.Open(config.Namespace)
	if err != nil {
		return err
	}
	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err = models.AutoMigrate(db); err != nil {
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

	network := "tcp"
	address := fmt.Sprintf("0.0.0.0:%d", port)
	log.Infof("start gRPC on %s://%s", network, address)
	socket, err := net.Listen(network, address)
	if err != nil {
		log.Fatalln(err)
	}
	var opts []grpc.ServerOption

	server := grpc.NewServer(opts...)

	pb.RegisterUserServer(server, services.NewUserService(db, cache, aes, mac, jwt, enforcer, &config.RabbitMq, s3))
	pb.RegisterPolicyServer(server, services.NewPolicyService(db, jwt, enforcer))
	pb.RegisterSmsServer(server, services.NewSmsService(db, jwt, enforcer, &config.RabbitMq))
	pb.RegisterEmailServer(server, services.NewEmailService(db, jwt, enforcer, &config.RabbitMq))
	pb.RegisterS3Server(server, services.NewS3Service(config.Namespace, db, jwt, enforcer, s3))
	pb.RegisterLocaleServer(server, services.NewLocaleService(db, jwt, enforcer))
	pb.RegisterLeaveWordServer(server, services.NewLeaveWordService(db, jwt, enforcer))
	pb.RegisterNotificationServer(server, services.NewNotificationService(db, jwt, enforcer))
	pb.RegisterTagServer(server, services.NewTagService(db, jwt, enforcer))
	pb.RegisterCategoryServer(server, services.NewCategoryService(db, jwt, enforcer))
	pb.RegisterSiteServer(server, services.NewSiteService(db, aes, jwt, enforcer))

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return server.Serve(socket)
}
