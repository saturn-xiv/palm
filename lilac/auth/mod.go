package auth

import (
	"context"
	"strings"

	"github.com/go-playground/validator/v10"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	"github.com/saturn-xiv/palm/lilac/models"
	rbac_pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

var (
	gl_validate *validator.Validate = validator.New(validator.WithRequiredStructEnabled())
)

func IsDomain(s string) (string, error) {
	v := strings.ToLower(strings.TrimSpace(s))
	if err := gl_validate.Var(s, "required,fqdn,min=3,max=127"); err != nil {
		return "", err
	}
	return v, nil
}

func IsPassword(s string) error {
	if err := gl_validate.Var(s, "required,min=6,max=31"); err != nil {
		return err
	}
	return nil
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
	ProviderType rbac_pb.UserDetail_Provider_Type
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

func CurrentUserFromToken(db *gorm.DB, jwt *crypto.Jwt, token string) (*CurrentUser, error) {
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
		return CurrentUserFromToken(db, jwt, token)
	}

	return nil, status.Errorf(codes.Unauthenticated, "authorization token is not provided")
}
