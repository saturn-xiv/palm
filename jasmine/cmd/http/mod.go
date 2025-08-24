package http

import (
	"embed"
	"encoding/base64"
	http_ "net/http"

	"github.com/gorilla/csrf"
	"github.com/gorilla/sessions"

	"github.com/saturn-xiv/palm/jasmine/env"
	redis_ "github.com/saturn-xiv/palm/jasmine/env/redis"
)

//go:embed all:templates
var gl_templates_fs embed.FS

//go:embed all:views
var gl_views_fs embed.FS

//go:embed all:assets
var gl_assets_fs embed.FS

type Config struct {
	// openssl rand -base64 32
	CookieSecret   string         `toml:"cookie-secret"`
	CsrfSecret     string         `toml:"csrf-secret"`
	AllowedOrigins []string       `toml:"allowed-origins"`
	SecretsStore   string         `toml:"secrets-store"`
	Database       env.Database   `toml:"database"`
	Redis          redis_.Cluster `toml:"redis"`
}

func (p *Config) OpenSessionStore() (sessions.Store, error) {
	buf, err := base64.StdEncoding.DecodeString(p.CookieSecret)
	if err != nil {
		return nil, err
	}
	store := sessions.NewCookieStore(buf)
	return store, nil
}

func (p *Config) OpenCsrf() (func(http_.Handler) http_.Handler, error) {
	buf, err := base64.StdEncoding.DecodeString(p.CsrfSecret)
	if err != nil {
		return nil, err
	}
	return csrf.Protect(buf), nil
}

func ReadHtmlTemplate(name string) ([]byte, error) {
	return gl_views_fs.ReadFile(name)
}
