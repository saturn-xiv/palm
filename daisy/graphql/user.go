package graphql

import "gorm.io/gorm"

type userProfile struct{}

type userProvider struct {
	Type string
	Id   uint
}

type SignInResponse struct{}

func newSignInResponse(db *gorm.DB, user *userProvider) (*SignInResponse, error) {
	// TODO
	return &SignInResponse{}, nil
}
