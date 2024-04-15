package rpc

import (
	"context"
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/BurntSushi/toml"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	"github.com/saturn-xiv/palm/lilac/services"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

func Launch(address string, config_file string, keys_dir string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
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
	slog.Info(fmt.Sprintf("start gRPC on %s://%s", network, address))
	socket, err := net.Listen(network, address)
	if err != nil {
		return err
	}
	var options []grpc.ServerOption

	server := grpc.NewServer(options...)

	if config.GoogleOauth2.ClientID != "" {
		slog.Warn("register google-oauth2 service")
		pb.RegisterGoogleOauth2Server(server, services.NewGoogleOauth2Service(db, jwt, enforcer))
	}
	if config.WechatOauth2.AppID != "" {
		slog.Warn("register wechat-oauth2 service")
		pb.RegisterWechatOauth2Server(server, services.NewWechatOauth2Service(db, jwt, enforcer))
	}
	if config.WechatMiniProgram.AppID != "" {
		slog.Warn("register wechat-mini-program service")
		pb.RegisterWechatMiniProgramServer(server, services.NewWechatMiniProgramService(db, jwt, enforcer))
	}
	if config.WeChatPayMerchant.ID != "" {
		slog.Warn("register wechat-pay service")
		ctx := context.Background()
		client, err := config.WeChatPayMerchant.Open(ctx)
		if err != nil {
			return err
		}
		pb.RegisterWechatPayServer(server, services.NewWechatPayService(db, jwt, enforcer, client))
	}

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
