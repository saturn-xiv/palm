package rpc

import (
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/BurntSushi/toml"
	"github.com/casbin/casbin/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"google.golang.org/grpc/reflection"
	"gorm.io/gorm"

	balsam_services "github.com/saturn-xiv/palm/atropa/balsam/services"
	balsam_pb "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat_mini_program "github.com/saturn-xiv/palm/atropa/env/wechat-mini-program"
	wechat_oauth2 "github.com/saturn-xiv/palm/atropa/env/wechat-oauth2"
	wechat_pay "github.com/saturn-xiv/palm/atropa/env/wechat-pay"
	google_services "github.com/saturn-xiv/palm/atropa/google/services"
	google_pb "github.com/saturn-xiv/palm/atropa/google/services/v2"
	rbac_services "github.com/saturn-xiv/palm/atropa/rbac/services"
	rbac_pb "github.com/saturn-xiv/palm/atropa/rbac/services/v2"
	s3_services "github.com/saturn-xiv/palm/atropa/s3/services"
	s3_pb "github.com/saturn-xiv/palm/atropa/s3/services/v2"
	wechat_services "github.com/saturn-xiv/palm/atropa/wechat/services"
	wechat_pb "github.com/saturn-xiv/palm/atropa/wechat/services/v2"
)

func Launch(port uint16, config_file string, keys_dir string, version string) error {
	slog.Debug(fmt.Sprintf("load configuration from %s", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err = db.Transaction(i18n_sync); err != nil {
		return err
	}
	redis, err := config.Redis.Open()
	if err != nil {
		return err
	}
	enforcer, err := env.OpenCasbinEnforcer(db, &config.Redis)
	if err != nil {
		return err
	}

	aes, hmac, jwt, err := crypto.Open(keys_dir)
	if err != nil {
		return err
	}
	s3, err := config.Minio.Open()
	if err != nil {
		return err
	}

	address := fmt.Sprintf("0.0.0.0:%d", port)
	network := "tcp"
	slog.Info(fmt.Sprintf("start gRPC on %s://%s", network, address))
	socket, err := net.Listen(network, address)
	if err != nil {
		return err
	}

	tls, err := config.Tls.Load()
	if err != nil {
		return err
	}

	server := grpc.NewServer(grpc.Creds(credentials.NewTLS(tls)))
	if err = mount(server,
		db, redis, &config.RabbitMQ,
		aes, hmac, jwt, enforcer, s3,
		config.GoogleOauth2,
		config.WechatOauth2, config.WechatMiniProgram, config.WechatPay); err != nil {
		return err
	}
	reflection.Register(server)

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

func mount(server *grpc.Server,
	db *gorm.DB, redis *redis.Client, rabbitmq *rabbitmq.Config,
	aes *crypto.Aes, hmac *crypto.HMac, jwt *crypto.Jwt,
	enforcer *casbin.Enforcer,
	s3 *minio.Cluster,
	google_oauth2 *GoogleOauth2,
	wechat_oauth2 *wechat_oauth2.Config,
	wechat_mini_program *wechat_mini_program.Config,
	wechat_pay *wechat_pay.Config,
) error {
	balsam_pb.RegisterAesServer(server, balsam_services.NewAesService(aes))
	balsam_pb.RegisterHMacServer(server, balsam_services.NewHmacService(hmac))
	balsam_pb.RegisterJwtServer(server, balsam_services.NewJwtService(jwt))
	balsam_pb.RegisterSessionServer(server, balsam_services.NewSessionService(db))
	balsam_pb.RegisterUserServer(server, balsam_services.NewUserService(db))
	balsam_pb.RegisterEmailUserServer(server, balsam_services.NewEmailUserService(db, rabbitmq, enforcer, hmac, jwt))
	balsam_pb.RegisterGoogleOauth2UserServer(server, balsam_services.NewGoogleOauth2Service(db))
	balsam_pb.RegisterWechatMiniProgramUserServer(server, balsam_services.NewWechatMiniProgramService(db))
	balsam_pb.RegisterWechatOauth2UserServer(server, balsam_services.NewWechatOauth2Service(db))
	balsam_pb.RegisterLocaleServer(server, balsam_services.NewLocaleService(db))
	balsam_pb.RegisterLeaveWordServer(server, balsam_services.NewLeaveWordService(db))
	balsam_pb.RegisterAttachmentServer(server, balsam_services.NewAttachmentService(db))
	rbac_pb.RegisterPolicyServer(server, rbac_services.NewPolicyService(enforcer))
	s3_pb.RegisterS3Server(server, s3_services.NewS3Service(s3))
	if google_oauth2 != nil {
		service, err := google_services.NewOauth2Service(jwt, google_oauth2.ProjectID, google_oauth2.RedirectURL)
		if err != nil {
			return err
		}
		google_pb.RegisterOauth2Server(server, service)
	}

	if wechat_oauth2 != nil {
		wechat_pb.RegisterOauth2Server(server, wechat_services.NewOauth2Service(jwt, wechat_oauth2))
	}
	if wechat_mini_program != nil {
		wechat_pb.RegisterMiniProgramServer(server, wechat_services.NewMiniProgramService(redis, wechat_mini_program))
	}
	if wechat_pay != nil {
		wechat_pb.RegisterPayBillServer(server, wechat_services.NewPayBillService(redis, wechat_pay))
	}
	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return nil
}
