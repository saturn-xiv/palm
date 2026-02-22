package graphql

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/graph-gophers/graphql-go"
	"golang.org/x/oauth2"
	"golang.org/x/text/language"
	google_oauth2 "google.golang.org/api/oauth2/v2"
	"google.golang.org/api/option"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

func (p *Mutation) SignInByGoogleOauth2(ctx context.Context, args struct {
	Home      string
	SessionId string
	Code      string
	State     string
	Lang      string
	Timezone  string
	Ttl       uint
}) (*UserSignInResponse, error) {
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
		var it string
		if err := p.redis.GetB(ctx, google_oauth2_session_key(args.SessionId), &it); err != nil {
			return nil, err
		}
		if args.State != it {
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
		return models.UserSignInByGoogleOauth2(tx, user_info, ip, &lang, timezone)
	}); err != nil {
		return nil, err
	}
	return newUserSignInResponse(p.db, v2.Session_GOOGLE_OAUTH2, user_info.Id, args.Ttl)
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

func (p *Query) IndexGoogleOauth2User(ctx context.Context, args struct {
	Page Page
}) (*IndexGoogleOauth2UserResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.GoogleOauth2User{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.GoogleOauth2User
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexGoogleOauth2UserResponse{db: p.db, items: items, pagination: pagination}, nil
}

type IndexGoogleOauth2UserResponse struct {
	db         *gorm.DB
	items      []models.GoogleOauth2User
	pagination *Pagination
}

func (p *IndexGoogleOauth2UserResponse) Items() []*GoogleOauth2User {
	var items []*GoogleOauth2User
	for _, it := range p.items {
		items = append(items, &GoogleOauth2User{item: &it, db: p.db})
	}
	return items
}
func (p *IndexGoogleOauth2UserResponse) Pagination() *Pagination {
	return p.pagination
}

type GoogleOauth2User struct {
	item *models.GoogleOauth2User
	db   *gorm.DB
}

func (p *GoogleOauth2User) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *GoogleOauth2User) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}
func (p *GoogleOauth2User) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *GoogleOauth2User) User() (*UserDetails, error) {
	var it models.User
	if err := p.db.Unscoped().First(&it, p.item.UserID).Error; err != nil {
		return nil, err
	}
	return &UserDetails{item: &it}, nil
}
func (p *GoogleOauth2User) Code() string {
	return p.item.Code
}
func (p *GoogleOauth2User) Name() string {
	return p.item.Name
}
func (p *GoogleOauth2User) Email() string {
	return p.item.Email
}
func (p *GoogleOauth2User) EmailVerified() *bool {
	return p.item.EmailVerified
}
func (p *GoogleOauth2User) Picture() string {
	return p.item.Picture
}
func (p *GoogleOauth2User) Gender() string {
	return p.item.Gender
}
func (p *GoogleOauth2User) Link() string {
	return p.item.Link
}
func (p *GoogleOauth2User) Locale() string {
	return p.item.Locale
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
