package env

import (
	"context"
	"fmt"
	"log/slog"

	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	oauth2_api "google.golang.org/api/oauth2/v2"
	"google.golang.org/api/option"
)

type GoogleOauth2 struct {
	ClientID     string `toml:"client-id"`
	ClientSecret string `toml:"client-secret"`
}

// https://developers.google.com/identity/protocols/oauth2/scopes#oauth2
func (p *GoogleOauth2) AuthUrl(domain string, state string) string {
	config := oauth2.Config{
		ClientID:     p.ClientID,
		ClientSecret: p.ClientID,
		Endpoint:     google.Endpoint,
		Scopes: []string{
			"https://www.googleapis.com/auth/userinfo.email",
			"https://www.googleapis.com/auth/userinfo.profile",
			"openid",
		},
		RedirectURL: fmt.Sprintf("https://%s/google-oauth2/callback", domain),
	}
	url := config.AuthCodeURL(state)
	slog.Debug(fmt.Sprintf("google redirect url: %s", url))
	return url
}

type GoogleClient struct {
	CredentialsFile string `toml:"credentials-file"`
}

func (p *GoogleClient) Oauth2Service(ctx context.Context, domain string) (*oauth2_api.Service, error) {
	return oauth2_api.NewService(ctx, option.WithCredentialsFile(p.CredentialsFile))
}
