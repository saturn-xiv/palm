package services

import (
	"context"
	"reflect"
	"strings"

	"github.com/casbin/casbin/v2"
	"github.com/go-playground/validator/v10"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

var (
	gl_validate *validator.Validate = validator.New(validator.WithRequiredStructEnabled())

	gl_email_validator_tag    = "required,email,lowercase,max=127"
	gl_code_validator_tag     = "required,alphanum,lowercase,min=2,max=31"
	gl_name_validator_tag     = "required,min=2,max=63"
	gl_password_validator_tag = "required,min=6,max=31"
	// gl_title_validator_tag    = "required,min=1,max=63"
	// gl_summary_validator_tag  = "required,min=2,max=511"
)

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
