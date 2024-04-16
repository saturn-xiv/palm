package services

import (
	"context"
	"log/slog"
	"reflect"
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

	gl_email_validator_tag    = "required,email,lowercase,max=127"
	gl_code_validator_tag     = "required,alphanum,lowercase,min=2,max=31"
	gl_name_validator_tag     = "required,min=2,max=63"
	gl_password_validator_tag = "required,min=6,max=31"
	gl_domain_validator_tag   = "required,fqdn,min=3,max=127"
	// gl_title_validator_tag    = "required,min=1,max=63"
	// gl_summary_validator_tag  = "required,min=2,max=511"
)

func IsDomain(s string) error {
	return gl_validate.Var(s, gl_domain_validator_tag)
}

type CurrentUser struct {
	ID           uint32
	ProviderType string
	ProviderId   uint32
}

func (p *CurrentUser) IsRoot(enforcer *casbin.Enforcer) error {
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Root_{},
	})
}

func (p *CurrentUser) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Administrator_{},
	})
}
func (p *CurrentUser) Has(enforcer *casbin.Enforcer, role string) error {
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
			Member: role,
		},
	})
}

func (p *CurrentUser) has(enforcer *casbin.Enforcer, role *pb.PolicyRolesResponse_Item) error {
	user := pb.PolicyUserRequest{Id: p.ID}
	user_c, err := user.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	role_c, err := role.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	ok, err := enforcer.HasRoleForUser(user_c, role_c)
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	if !ok {
		return status.Errorf(codes.PermissionDenied, "user didn't have role of %s", role.Display())
	}
	return nil
}
func (p *CurrentUser) Can(enforcer *casbin.Enforcer, operation string, resource_type string) error {
	return p.can(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
		},
	})
}

func (p *CurrentUser) Can_(enforcer *casbin.Enforcer, operation string, resource_type string, resource_id uint32) error {
	return p.can(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
			Id:   &resource_id,
		},
	})
}

func (p *CurrentUser) can(enforcer *casbin.Enforcer, permission *pb.PolicyPermissionsResponse_Item) error {
	user := pb.PolicyUserRequest{Id: p.ID}
	subject, err := user.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	object, err := permission.Resource.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}

	items, err := enforcer.GetImplicitPermissionsForUser(subject)
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	for _, it := range items {
		if len(it) != 3 {
			continue
		}
		if it[1] == object && it[2] == permission.Operation {
			return nil
		}
	}
	return status.Errorf(codes.PermissionDenied, "user didn't have permission(%s, %s)", permission.Operation, permission.Resource.Display())
}

const (
	users_SIGN_IN_AUDIENCE = "users.sign-in"
)

func CurrentUserFromToken(token string, db *gorm.DB, jwt *crypto.Jwt) (*CurrentUser, error) {
	issuer := reflect.TypeOf(CurrentUser{}).PkgPath()
	_, subject, _, err := jwt.Verify(token, issuer, users_SIGN_IN_AUDIENCE)
	if err != nil {
		return nil, err
	}
	var it models.Session

	if rst := db.Where("uid = @uid", map[string]interface{}{"uid": subject}).First(&it); rst.Error != nil {
		return nil, rst.Error
	}

	return &CurrentUser{
		ID:           it.UserID,
		ProviderType: it.ProviderType,
		ProviderId:   it.ProviderId,
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
