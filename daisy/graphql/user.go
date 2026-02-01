package graphql

import (
	"context"
	"errors"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
)

type SignInResponse struct{}

func newSignInResponse(db *gorm.DB, provider_type v2.Session_ProviderType, provider_sn string) (*SignInResponse, error) {
	// TODO
	return &SignInResponse{}, nil
}

func CurrentUser(ctx context.Context, db *gorm.DB, jwt *crypto.Jwt) (*models.User, *v2.Session, error) {
	auth, ok := ctx.Value(headerKey(rbac.Authorization)).(string)
	if !ok {
		return nil, nil, errors.New("no authorization header")
	}
	return rbac.NewCurrentUserByAuthorization(auth, db, jwt)
}
