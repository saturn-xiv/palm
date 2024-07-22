package services

import (
	"bytes"
	"context"
	"encoding/gob"
	"time"

	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/crypto"
	google_oauth2 "github.com/saturn-xiv/palm/atropa/env/google-oauth2"
	pb "github.com/saturn-xiv/palm/atropa/google/services/v2"
)

var gl_google_oauth2_audience = "google.oauth2"

func NewOauth2Service(jwt *crypto.Jwt, project string, redirect_url string) (*Oauth2Service, error) {
	google, err := google_oauth2.NewClient(project, redirect_url)
	if err != nil {
		return nil, err
	}
	return &Oauth2Service{google: google}, nil
}

type Oauth2Service struct {
	pb.UnimplementedOauth2Server

	google *google_oauth2.Client
	jwt    *crypto.Jwt
}

func (p *Oauth2Service) AuthCodeURL(ctx context.Context, req *pb.Oauth2AuthCodeURLRequest) (*pb.Oauth2AuthCodeURLResponse, error) {
	now := time.Now()
	exp := now.Add(time.Minute * 5)

	subject := ""
	if req.Subject != nil {
		subject = *req.Subject
	}
	token, err := p.jwt.Sign(env.JWT_ISSUER, subject, []string{gl_google_oauth2_audience}, map[string]interface{}{}, &now, &exp)
	if err != nil {
		return nil, err
	}

	return &pb.Oauth2AuthCodeURLResponse{
		Url: p.google.AuthCodeURL(token),
	}, nil
}

func (p *Oauth2Service) SignIn(ctx context.Context, req *pb.Oauth2SignInRequest) (*pb.Oauth2SignInResponse, error) {
	_, subject, _, err := p.jwt.Verify(req.State, env.JWT_ISSUER, gl_google_oauth2_audience)
	if err != nil {
		return nil, err
	}

	token, id_token, err := p.google.Exchange(ctx, req.Code)
	if err != nil {
		return nil, err
	}

	var token_buf bytes.Buffer
	{
		enc := gob.NewEncoder(&token_buf)
		if err = enc.Encode(token); err != nil {
			return nil, err
		}
	}

	it := pb.Oauth2SignInResponse{
		OpenId: &pb.Oauth2SignInResponse_OpenId{
			Subject:       id_token.Subject,
			Email:         id_token.Email,
			EmailVerified: id_token.EmailVerified,
			Picture:       id_token.Picture,
			Profile:       id_token.Profile,
			Locale:        id_token.Locale,
		},
		Token: token_buf.Bytes(),
	}
	if subject != "" {
		it.Subject = &subject
	}
	return &it, nil
}
