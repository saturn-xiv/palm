package env

import (
	"context"
	"fmt"

	log "github.com/sirupsen/logrus"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	oauth2_api "google.golang.org/api/oauth2/v2"
	"google.golang.org/api/option"
)

type GoogleOauth2 struct {
	ClientID     string   `toml:"client-id"`
	ClientSecret string   `toml:"client-secret"`
	Scopes       []string `toml:"scopes"`
}

func (p *GoogleOauth2) AuthUrl(domain string, state string) string {
	config := oauth2.Config{
		ClientID:     p.ClientID,
		ClientSecret: p.ClientID,
		Endpoint:     google.Endpoint,
		Scopes:       p.Scopes,
		RedirectURL:  fmt.Sprintf("https://%s/google-oauth2/callback", domain),
	}
	url := config.AuthCodeURL(state)
	log.Debugf("google redirect url: %s", url)
	return url
}

type GoogleClient struct {
	CredentialsFile string `toml:"credentials-file"`
}

func (p *GoogleClient) Oauth2Service(ctx context.Context, domain string) (*oauth2_api.Service, error) {
	return oauth2_api.NewService(ctx, option.WithCredentialsFile(p.CredentialsFile))
}
