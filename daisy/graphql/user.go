package graphql

import (
	"context"
	"errors"
	"fmt"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

const (
	userSignInAudience = "user.sign-in"
)

func CurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*models.User, *v2.Session, error) {
	token, ok := ctx.Value(headerKey(Authorization)).(string)
	if !ok {
		return nil, nil, errors.New("no token")
	}
	_, sub, err := jwt.Verify(token, env.Plugin(), userSignInAudience)
	if err != nil {
		return nil, nil, err
	}

	ss, err := v2.NewSession(sub)
	if err != nil {
		return nil, nil, err
	}
	user, err := current_user(db, ss)
	if err != nil {
		return nil, nil, err
	}
	return user, ss, nil
}

func current_user(db *gorm.DB, ss *v2.Session) (*models.User, error) {
	var user models.User
	switch ss.Type {
	case v2.User_GOOGLE_OAUTH2:
		var it models.GoogleOauth2User
		if err := db.Where("sn = ?", ss.Sn).Preload("User").First(&it).Error; err != nil {
			return nil, err
		}
		if it.User.LockedAt != nil {
			return nil, fmt.Errorf("user %s is locked", it.Name)
		}
	default:
		return nil, fmt.Errorf("unsupported %s yet", ss.Type.String())
	}

	if user.LockedAt != nil {
		return nil, errors.New("user is locked")
	}
	return &user, nil
}

type SignInResponse struct{}

func newSignInResponse(db *gorm.DB, provider_type v2.User_ProviderType, provider_sn string) (*SignInResponse, error) {
	// TODO
	return &SignInResponse{}, nil
}
