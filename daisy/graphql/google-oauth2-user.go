package graphql

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"golang.org/x/oauth2"
	"golang.org/x/text/language"
	google_oauth2 "google.golang.org/api/oauth2/v2"
	"google.golang.org/api/option"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

func (p *Mutation) SignInByGoogleOauth2(ctx context.Context, args struct {
	Home      string
	Code      string
	SessionId string
	State     string
	Lang      string
	Timezone  string
}) (*SignInResponse, error) {
	ip := ClientIp(ctx)
	timezone, err := time.LoadLocation(args.Timezone)
	if err != nil {
		return nil, err
	}
	lang, err := language.Parse(args.Lang)
	if err != nil {
		return nil, err
	}
	home := strings.TrimSpace(strings.ToLower(args.Home))
	{
		if err := gl_validate.Struct(&googleOauth2HomeForm{Url: home}); err != nil {
			return nil, err
		}
	}
	{
		var state string
		if err := p.redis.GetB(ctx, google_oauth2_session_key(args.SessionId), &state); err != nil {
			return nil, err
		}
		if args.State != state {
			return nil, errors.New("invalid state")
		}
	}

	cfg := p.google_oauth2.Open(home)
	token, err := cfg.Exchange(ctx, args.Code)
	if err != nil {
		return nil, err
	}

	service, err := google_oauth2.NewService(ctx, option.WithTokenSource(cfg.TokenSource(ctx, token)))
	if err != nil {
		return nil, err
	}
	user_service := google_oauth2.NewUserinfoService(service)
	user_info, err := user_service.Get().Do()
	if err != nil {
		return nil, err
	}

	if err = p.db.Transaction(func(tx *gorm.DB) error {
		err := models.UserSignInByGoogleOauth2(tx, user_info, &lang, timezone)
		if err != nil {
			return err
		}
		var it models.GoogleOauth2User
		if err := tx.Where("code = ?", user_info.Id).Preload("User").First(&it).Error; err != nil {
			return err
		}
		if it.User.LockedAt != nil {
			return fmt.Errorf("user %s is locked", user_info.Name)
		}
		if err = models.SignInUser(tx, it.User, ip); err != nil {
			return err
		}
		if err = models.CreateLog(tx, it.UserID, env.Plugin(), ip, v2.Log_INFO, "sign in by google oauth2"); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return newSignInResponse(p.db, v2.Session_GOOGLE_OAUTH2, user_info.Id)
}

func (p *Query) GetGoogleOauth2Url(ctx context.Context, args struct {
	Home      string
	SessionId string
}) (string, error) {
	home := strings.TrimSpace(strings.ToLower(args.Home))
	{
		if err := gl_validate.Struct(&googleOauth2HomeForm{Url: home}); err != nil {
			return "", err
		}
	}
	state, err := random_alphanumeric(8)
	if err != nil {
		return "", err
	}
	if err = p.redis.SetB(ctx, google_oauth2_session_key(args.SessionId), state, time.Minute*5); err != nil {
		return "", err
	}
	cfg := p.google_oauth2.Open(home)
	return cfg.AuthCodeURL(state), nil
}

// https://developers.google.com/identity/protocols/oauth2/web-server
type GoogleOauth2Config interface {
	Open(home string) *oauth2.Config
}

func google_oauth2_session_key(s string) string {
	return fmt.Sprintf("google.oauth2.session/%s", s)
}

type googleOauth2HomeForm struct {
	Url string `validate:"required,gte=12,lte=31,https_url"`
}
