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
	"github.com/casbin/casbin/v2"
	"github.com/minio/minio-go/v7"
	"google.golang.org/grpc"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/auth"
	auth_pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/email"
	email_pb "github.com/saturn-xiv/palm/lilac/email/v2"
	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	google_oauth2 "github.com/saturn-xiv/palm/lilac/google/oauth2"
	google_oauth2_pb "github.com/saturn-xiv/palm/lilac/google/oauth2/v2"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
	"github.com/saturn-xiv/palm/lilac/rbac"
	rbac_pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
	"github.com/saturn-xiv/palm/lilac/s3"
	s3_pb "github.com/saturn-xiv/palm/lilac/s3/v2"
	"github.com/saturn-xiv/palm/lilac/sms"
	sms_pb "github.com/saturn-xiv/palm/lilac/sms/v2"
	wechat_mini_program "github.com/saturn-xiv/palm/lilac/wechat/mini-program"
	wechat_mini_program_pb "github.com/saturn-xiv/palm/lilac/wechat/mini-program/v2"
	wechat_oauth2 "github.com/saturn-xiv/palm/lilac/wechat/oauth2"
	wechat_oauth2_pb "github.com/saturn-xiv/palm/lilac/wechat/oauth2/v2"
	wechat_pay "github.com/saturn-xiv/palm/lilac/wechat/pay"
	wechat_pay_pb "github.com/saturn-xiv/palm/lilac/wechat/pay/v2"
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

	mount(server,
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

func mount(server *grpc.Server, namespace string,
	db *gorm.DB, cache *redis.Client,
	aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt, enforcer *casbin.Enforcer,
	i18n *i18n.I18n, queue *rabbitmq.Config, s3c *minio.Client,
	google_oauth2_c *env.GoogleOauth2,
	wechat_oauth2_c *env.WechatOauth2, wechat_mini_program_c *env.WechatMiniProgram, wechat_pay_merchant_c *env.WechatPayMerchant,
) error {
	auth_pb.RegisterUserServer(server, auth.NewUserService(db, cache, aes, mac, jwt, enforcer, i18n, queue, s3c))
	auth_pb.RegisterLocaleServer(server, auth.NewLocaleService(db, jwt, enforcer))
	auth_pb.RegisterLeaveWordServer(server, auth.NewLeaveWordService(db, jwt, enforcer))
	auth_pb.RegisterNotificationServer(server, auth.NewNotificationService(db, jwt, enforcer))
	auth_pb.RegisterTagServer(server, auth.NewTagService(db, jwt, enforcer))
	auth_pb.RegisterCategoryServer(server, auth.NewCategoryService(db, jwt, enforcer))
	auth_pb.RegisterSiteServer(server, auth.NewSiteService(db, aes, jwt, enforcer))
	rbac_pb.RegisterPolicyServer(server, rbac.NewService(db, jwt, enforcer))
	sms_pb.RegisterSmsServer(server, sms.NewService(db, jwt, enforcer, queue))
	email_pb.RegisterEmailServer(server, email.NewService(db, jwt, enforcer, queue))
	s3_pb.RegisterS3Server(server, s3.NewService(namespace, db, jwt, enforcer, s3c))

	if google_oauth2_c.ClientID != "" {
		slog.Warn("register google-oauth2 service")
		google_oauth2_pb.RegisterGoogleOauth2Server(server, google_oauth2.NewService(db, jwt, enforcer))
	}
	if wechat_oauth2_c.AppID != "" {
		slog.Warn("register wechat-oauth2 service")
		wechat_oauth2_pb.RegisterWechatOauth2Server(server, wechat_oauth2.NewService(db, jwt, enforcer))
	}
	if wechat_mini_program_c.AppID != "" {
		slog.Warn("register wechat-mini-program service")
		wechat_mini_program_pb.RegisterWechatMiniProgramServer(server, wechat_mini_program.NewService(db, jwt, enforcer))
	}
	if wechat_pay_merchant_c.ID != "" {
		slog.Warn("register wechat-pay service")
		ctx := context.Background()
		client, err := wechat_pay_merchant_c.Open(ctx)
		if err != nil {
			return err
		}
		wechat_pay_pb.RegisterWechatPayJsapiServer(server, wechat_pay.NewJsapiService(db, jwt, enforcer, client))
		wechat_pay_pb.RegisterWechatPayNativeServer(server, wechat_pay.NewNativeService(db, jwt, enforcer, client))
		wechat_pay_pb.RegisterWechatPayBillServer(server, wechat_pay.NewBillService(db, jwt, enforcer, client))
		wechat_pay_pb.RegisterWechatPayRefundServer(server, wechat_pay.NewRefundService(db, jwt, enforcer, client))
		wechat_pay_pb.RegisterWechatPayTransferServer(server, wechat_pay.NewTransferService(db, jwt, enforcer, client))
	}

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return nil
}
