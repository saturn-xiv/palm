package services

import (
	"context"
	"log/slog"
	"strings"

	"github.com/casbin/casbin/v2"
	"github.com/go-playground/validator/v10"
	"github.com/minio/minio-go/v7"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/health"
	"google.golang.org/grpc/health/grpc_health_v1"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/env/rabbitmq"
	"github.com/saturn-xiv/palm/lilac/env/redis"
	"github.com/saturn-xiv/palm/lilac/i18n"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

var (
	gl_validate *validator.Validate = validator.New(validator.WithRequiredStructEnabled())

	gl_password_validator_tag = "required,min=6,max=31"
)

func IsDomain(s string) (string, error) {
	v := strings.ToLower(strings.TrimSpace(s))
	if err := gl_validate.Var(s, "required,fqdn,min=3,max=127"); err != nil {
		return "", err
	}
	return v, nil
}

func IsEmail(s string) (string, error) {
	v := strings.ToLower(strings.TrimSpace(s))
	if err := gl_validate.Var(s, "required,email,lowercase,max=127"); err != nil {
		return "", err
	}
	return v, nil
}

func IsUrl(s string) (string, error) {
	v := strings.TrimSpace(s)
	if err := gl_validate.Var(s, "required,url,max=127"); err != nil {
		return "", err
	}
	return v, nil
}
func IsCode(s string) (string, error) {
	v := strings.ToLower(strings.TrimSpace(s))
	if err := gl_validate.Var(s, "required,alphanum,lowercase,min=2,max=31"); err != nil {
		return "", err
	}
	return v, nil
}

func IsUsername(s string) (string, error) {
	v := strings.TrimSpace(s)
	if err := gl_validate.Var(s, "required,min=2,max=63"); err != nil {
		return "", err
	}
	return v, nil
}

type CurrentUser struct {
	Payload      *models.User
	ProviderType pb.UserIndexResponse_Item_ProviderType
	ProviderId   uint32
	Session      string
}

const (
	gl_jwt_issuer                       = "palm.lilac"
	gl_jwt_audience_user_sign_in        = "user.sign-in"
	gl_jwt_audience_user_confirm        = "user.confirm"
	gl_jwt_audience_user_unlock         = "user.unlock"
	gl_jwt_audience_user_reset_password = "user.reset-password"
)

func CurrentUserFromToken(token string, db *gorm.DB, jwt *crypto.Jwt) (*CurrentUser, error) {
	_, subject, _, err := jwt.Verify(token, gl_jwt_issuer, gl_jwt_audience_user_sign_in)
	if err != nil {
		return nil, err
	}
	var ss models.Session
	if rst := db.Where(&models.Session{Uid: subject}).First(&ss); rst.Error != nil {
		return nil, rst.Error
	}
	it, ud, err := models.UserFromSession(db, &ss)
	if err != nil {
		return nil, err
	}

	return &CurrentUser{
		Payload:      it,
		ProviderType: ud.ProviderType,
		ProviderId:   ss.ProviderId,
		Session:      ss.Uid,
	}, nil
}

func NewCurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*CurrentUser, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, status.Errorf(codes.Unauthenticated, "metadata is not provided")
	}

	for _, auth := range md.Get("authorization") {
		token, ok := strings.CutPrefix(auth, "Bearer ")
		if !ok {
			continue
		}
		return CurrentUserFromToken(token, db, jwt)
	}

	return nil, status.Errorf(codes.Unauthenticated, "authorization token is not provided")
}

func Mount(server *grpc.Server, namespace string,
	db *gorm.DB, cache *redis.Client,
	aes *crypto.Aes, mac *crypto.HMac, jwt *crypto.Jwt, enforcer *casbin.Enforcer,
	i18n *i18n.I18n, queue *rabbitmq.Config, s3 *minio.Client,
	google_oauth2 *env.GoogleOauth2,
	wechat_oauth2 *env.WechatOauth2, wechat_mini_program *env.WechatMiniProgram, wechat_pay_merchant *env.WechatPayMerchant,
) error {
	pb.RegisterUserServer(server, NewUserService(db, cache, aes, mac, jwt, enforcer, i18n, queue, s3))
	pb.RegisterPolicyServer(server, NewPolicyService(db, jwt, enforcer))
	pb.RegisterSmsServer(server, NewSmsService(db, jwt, enforcer, queue))
	pb.RegisterEmailServer(server, NewEmailService(db, jwt, enforcer, queue))
	pb.RegisterS3Server(server, NewS3Service(namespace, db, jwt, enforcer, s3))
	pb.RegisterLocaleServer(server, NewLocaleService(db, jwt, enforcer))
	pb.RegisterLeaveWordServer(server, NewLeaveWordService(db, jwt, enforcer))
	pb.RegisterNotificationServer(server, NewNotificationService(db, jwt, enforcer))
	pb.RegisterTagServer(server, NewTagService(db, jwt, enforcer))
	pb.RegisterCategoryServer(server, NewCategoryService(db, jwt, enforcer))
	pb.RegisterSiteServer(server, NewSiteService(db, aes, jwt, enforcer))

	if google_oauth2.ClientID != "" {
		slog.Warn("register google-oauth2 service")
		pb.RegisterGoogleOauth2Server(server, NewGoogleOauth2Service(db, jwt, enforcer))
	}
	if wechat_oauth2.AppID != "" {
		slog.Warn("register wechat-oauth2 service")
		pb.RegisterWechatOauth2Server(server, NewWechatOauth2Service(db, jwt, enforcer))
	}
	if wechat_mini_program.AppID != "" {
		slog.Warn("register wechat-mini-program service")
		pb.RegisterWechatMiniProgramServer(server, NewWechatMiniProgramService(db, jwt, enforcer))
	}
	if wechat_pay_merchant.ID != "" {
		slog.Warn("register wechat-pay service")
		ctx := context.Background()
		client, err := wechat_pay_merchant.Open(ctx)
		if err != nil {
			return err
		}
		pb.RegisterWechatPayServer(server, NewWechatPayService(db, jwt, enforcer, client))
	}

	grpc_health_v1.RegisterHealthServer(server, health.NewServer())
	return nil
}
