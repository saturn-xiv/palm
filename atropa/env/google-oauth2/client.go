package google_oauth2

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"net/url"
	"os"

	"github.com/coreos/go-oidc/v3/oidc"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/urlshortener/v1"
)

// https://developers.google.com/identity/protocols/oauth2
// https://developers.google.com/identity/protocols/oauth2/web-server
// https://developers.google.com/identity/openid-connect/openid-connect
type Client struct {
	config   *oauth2.Config
	verifier string
}

func NewClient(project string, redirect_url string) (*Client, error) {
	slog.Info("load google oauth2 config", slog.String("project", project))
	file, err := os.Open(fmt.Sprintf("%s.json", project))
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var cfg Config
	dec := json.NewDecoder(file)
	if err = dec.Decode(&cfg); err != nil {
		return nil, err
	}
	slog.Debug("got google oauth2", slog.String("client-id", cfg.Web.ClientId))

	return &Client{
		config: &oauth2.Config{
			ClientID:     cfg.Web.ClientId,
			ClientSecret: cfg.Web.ClientSecret,
			Endpoint:     google.Endpoint,
			Scopes: []string{
				urlshortener.UrlshortenerScope,
				"https://www.googleapis.com/auth/userinfo.email",
				"https://www.googleapis.com/auth/userinfo.profile",
				oidc.ScopeOpenID, "profile", "email",
			},
			RedirectURL: redirect_url,
		},
		verifier: oauth2.GenerateVerifier(),
	}, nil
}

func (p *Client) AuthCodeURL(state string) string {
	return p.config.AuthCodeURL(state, oauth2.AccessTypeOffline, oauth2.S256ChallengeOption(p.verifier))
}

func (p *Client) Exchange(ctx context.Context, code string) (*oauth2.Token, *IdToken, error) {
	token, err := p.config.Exchange(ctx, code, oauth2.VerifierOption(p.verifier))
	if err != nil {
		return nil, nil, err
	}
	var id_token IdToken
	{
		provider, err := oidc.NewProvider(ctx, "https://accounts.google.com")
		if err != nil {
			return nil, nil, err
		}
		verifier := provider.Verifier(&oidc.Config{ClientID: p.config.ClientID})
		raw_id_token, ok := token.Extra("id_token").(string)
		if !ok {
			return nil, nil, errors.New("empty id token field")
		}
		bin_id_token, err := verifier.Verify(ctx, raw_id_token)
		if err != nil {
			return nil, nil, err
		}
		if err = bin_id_token.Claims(&id_token); err != nil {
			return nil, nil, err
		}
	}
	return token, &id_token, nil
}

func (p *Client) Revoke(ctx context.Context, token *oauth2.Token) error {
	client := p.config.Client(ctx, token)
	res, err := client.PostForm("https://oauth2.googleapis.com/revoke", url.Values{"token": {token.AccessToken}})
	if err != nil {
		return err
	}
	defer res.Body.Close()
	if res.StatusCode != http.StatusOK {
		body, err := io.ReadAll(res.Body)
		if err != nil {
			return err
		}
		slog.Error("revoke token", slog.String("reason", string(body)))
		return errors.New(res.Status)
	}

	return nil
}
