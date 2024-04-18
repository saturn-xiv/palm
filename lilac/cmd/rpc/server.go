package rpc

import (
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
	"github.com/saturn-xiv/palm/lilac/services"
)

func Launch(address string, config_file string, keys_dir string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err = models.AutoMigrate(db); err != nil {
		return err
	}
	cache, err := config.Redis.Open(config.Namespace)
	if err != nil {
		return err
	}
	i18n, err := i18n.New(db)
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

	network := "tcp"
	slog.Info(fmt.Sprintf("start gRPC on %s://%s", network, address))
	socket, err := net.Listen(network, address)
	if err != nil {
		return err
	}
	var options []grpc.ServerOption

	server := grpc.NewServer(options...)

	services.Mount(server,
		config.Namespace, db, cache,
		aes, mac, jwt, enforcer,
		i18n, &config.RabbitMq, s3,
		&config.GoogleOauth2,
		&config.WechatOauth2, &config.WechatMiniProgram, &config.WeChatPayMerchant,
	)

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
