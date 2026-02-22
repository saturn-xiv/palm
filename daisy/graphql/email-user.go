package graphql

import (
	"context"
	"errors"
	"fmt"
	"reflect"
	"strings"
	"time"

	"gorm.io/gorm"

	"github.com/graph-gophers/graphql-go"
	"github.com/saturn-xiv/palm/daisy/crypto"
	email_v2 "github.com/saturn-xiv/palm/daisy/email/v2"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	portal_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/queue"
)

const (
	emailUserResetPasswordAudience = "email-user.reset-password"
	emailUserConfirmAudience       = "email-user.confirm"
	emailUserUnlockAudience        = "email-user.unlock"
)

func (p *Mutation) ChangeEmailUserPassword(ctx context.Context, args struct {
	CurrentPassword string
	NewPassword     string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != portal_v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		if err := it.VerifyPassword(p.hmac, args.CurrentPassword); err != nil {
			return err
		}
		form := models.NewSetPasswordForEmailUserForm(args.NewPassword)
		if err := form.Execute(p.db, &it, p.hmac); err != nil {
			return err
		}
		return models.CreateLog(tx, uint(ss.User.Id), portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_WARNING, "change password")
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}
func (p *Mutation) SetEmailUserAvatar(ctx context.Context, args struct {
	Avatar string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != portal_v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	form := setEmailUserAvatarForm{Avatar: strings.TrimSpace(args.Avatar)}
	if err := gl_validate.Struct(&form); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		if err = tx.Model(&it).Updates(map[string]interface{}{
			"avatar": form.Avatar,
		}).Error; err != nil {
			return err
		}

		return models.CreateLog(tx, uint(ss.User.Id), portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_INFO, "update avatar")
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

type setEmailUserAvatarForm struct {
	Avatar string `validate:"required,min=5,max=127,url"`
}

func (p *Mutation) SetEmailUserName(ctx context.Context, args struct {
	Name string
}) (*Ok, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if ss.Subject.Type != portal_v2.Session_EMAIL {
		return nil, errors.New("not an email user")
	}

	form := setEmailUserNameForm{Name: strings.TrimSpace(args.Name)}
	if err := gl_validate.Struct(&form); err != nil {
		return nil, err
	}

	if err := p.db.Transaction(func(tx *gorm.DB) error {
		var it models.EmailUser
		if err := tx.Where("email = ?", ss.Subject.Sn).First(&it).Error; err != nil {
			return err
		}
		if err = tx.Model(&it).Updates(map[string]interface{}{
			"name": form.Name,
		}).Error; err != nil {
			return err
		}

		return models.CreateLog(tx, uint(ss.User.Id), portal_v2.Plugin(), ss.ClientIp, portal_v2.UserIndexLogResponse_Item_INFO, "update name")
	}); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

type setEmailUserNameForm struct {
	Name string `validate:"required,min=2,max=31"`
}

func (p *Mutation) SignInByEmail(ctx context.Context, args struct {
	Email    string
	Password string
	Ttl      uint
}) (*UserSignInResponse, error) {
	ip := ClientIp(ctx)
	form := models.NewEmailUserSignInForm(args.Email, args.Password)
	if err := p.db.Transaction(func(tx *gorm.DB) error {
		return form.Execute(tx, p.hmac, ip)
	}); err != nil {
		return nil, err
	}
	return newUserSignInResponse(p.db, portal_v2.Session_EMAIL, form.Email, args.Ttl)
}

func (p *Query) ForgotEmailUserPassword(ctx context.Context, args struct {
	Home  string
	Email string
}) (*Ok, error) {
	home := strings.TrimSpace(strings.ToLower(args.Home))
	email := strings.TrimSpace(strings.ToLower(args.Email))
	{
		if err := gl_validate.Struct(&emailUserForm{Home: home, Email: email}); err != nil {
			return nil, err
		}
	}

	var user models.EmailUser
	if err := p.db.Where("email = ?", email).Preload("User").First(&user).Error; err != nil {
		return nil, err
	}
	if user.ConfirmedAt == nil {
		return nil, fmt.Errorf("user %s isn't confirmed yet", email)
	}
	if user.User.LockedAt != nil {
		return nil, fmt.Errorf("user %s is locked", email)
	}

	if err := send_email(ctx, p.db, p.rabbitmq, p.jwt, home, &user, emailUserResetPasswordAudience); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

type emailUserForm struct {
	Home  string `validate:"required,gte=12,lte=31,https_url"`
	Email string `validate:"required,gte=5,lte=31,email"`
}

func (p *Query) ConfirmEmailUser(ctx context.Context, args struct {
	Home  string
	Email string
}) (*Ok, error) {
	home := strings.TrimSpace(strings.ToLower(args.Home))
	email := strings.TrimSpace(strings.ToLower(args.Email))
	{
		if err := gl_validate.Struct(&emailUserForm{Home: home, Email: email}); err != nil {
			return nil, err
		}
	}

	var user models.EmailUser
	if err := p.db.Where("email = ?", email).Preload("User").First(&user).Error; err != nil {
		return nil, err
	}
	if user.ConfirmedAt != nil {
		return nil, fmt.Errorf("user %s already confirmed", email)
	}
	if user.User.LockedAt != nil {
		return nil, fmt.Errorf("user %s is locked", email)
	}

	if err := send_email(ctx, p.db, p.rabbitmq, p.jwt, home, &user, emailUserConfirmAudience); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Query) UnlockEmailUser(ctx context.Context, args struct {
	Home  string
	Email string
}) (*Ok, error) {
	home := strings.TrimSpace(strings.ToLower(args.Home))
	email := strings.TrimSpace(strings.ToLower(args.Email))
	{
		if err := gl_validate.Struct(&emailUserForm{Home: home, Email: email}); err != nil {
			return nil, err
		}
	}

	var user models.EmailUser
	if err := p.db.Where("email = ?", email).Preload("User").First(&user).Error; err != nil {
		return nil, err
	}
	if user.ConfirmedAt == nil {
		return nil, fmt.Errorf("user %s isn't confirmed yet", email)
	}
	if user.User.LockedAt == nil {
		return nil, fmt.Errorf("user %s isn't locked", email)
	}

	if err := send_email(ctx, p.db, p.rabbitmq, p.jwt, home, &user, emailUserUnlockAudience); err != nil {
		return nil, err
	}
	return &Ok{}, nil
}

func (p *Query) IndexEmailUser(ctx context.Context, args struct {
	Page Page
}) (*IndexEmailUserResponse, error) {
	ss, err := CurrentUser(ctx, p.db, p.jwt)
	if err != nil {
		return nil, err
	}
	if err := ss.User.IsAdministrator(p.enforcer); err != nil {
		return nil, err
	}
	var total int64
	if err := p.db.Model(&models.EmailUser{}).Count(&total).Error; err != nil {
		return nil, err
	}
	pagination := NewPagination(&args.Page, uint(total))
	var items []models.EmailUser
	if err := p.db.Offset(int(pagination.current.Offset())).Limit(int(pagination.current.Size)).Order("updated_at DESC").Find(&items).Error; err != nil {
		return nil, err
	}
	return &IndexEmailUserResponse{db: p.db, items: items, pagination: pagination}, nil
}

type IndexEmailUserResponse struct {
	db         *gorm.DB
	items      []models.EmailUser
	pagination *Pagination
}

func (p *IndexEmailUserResponse) Items() []*EmailUser {
	var items []*EmailUser
	for _, it := range p.items {
		items = append(items, &EmailUser{item: &it, db: p.db})
	}
	return items
}
func (p *IndexEmailUserResponse) Pagination() *Pagination {
	return p.pagination
}

type EmailUser struct {
	item *models.EmailUser
	db   *gorm.DB
}

func (p *EmailUser) Id() graphql.ID {
	return ToId(p.item.ID)
}
func (p *EmailUser) CreatedAt() graphql.Time {
	return graphql.Time{Time: p.item.CreatedAt}
}
func (p *EmailUser) UpdatedAt() graphql.Time {
	return graphql.Time{Time: p.item.UpdatedAt}
}
func (p *EmailUser) User() (*UserDetails, error) {
	var it models.User
	if err := p.db.Unscoped().First(&it, p.item.UserID).Error; err != nil {
		return nil, err
	}
	return &UserDetails{item: &it}, nil
}
func (p *EmailUser) ConfirmedAt() *graphql.Time {
	if p.item.ConfirmedAt == nil {
		return nil
	}
	return &graphql.Time{Time: *p.item.ConfirmedAt}
}
func (p *EmailUser) Email() string {
	return p.item.Email
}
func (p *EmailUser) Name() string {
	return p.item.Name
}

func send_email(ctx context.Context, db *gorm.DB, queue *queue.RabbitMQ, jwt *crypto.Jwt, home string, user *models.EmailUser, audience string) error {
	token, err := jwt.Sign(env.Plugin(), user.Email, []string{audience}, time.Hour*1)
	if err != nil {
		return err
	}
	args := map[string]interface{}{
		"name":  user.Name,
		"home":  home,
		"token": token,
	}
	plugin := env.Plugin()
	subject := models.T(db, user.User.Lang, fmt.Sprintf("%s.mailer.%s.subject", plugin, audience), args)
	body := models.T(db, user.User.Lang, fmt.Sprintf("%s.mailer.%s.body", plugin, audience), args)

	if err = queue.ProduceProtobuf(ctx, reflect.TypeOf((*email_v2.Task)(nil)).Elem().Name(), &email_v2.Task{
		To:      &email_v2.Task_Address{Name: user.Name, Email: user.Email},
		Subject: subject,
		Body: &email_v2.Task_Body{
			Content: body,
			Html:    true,
		},
	}); err != nil {
		return err
	}
	return nil
}
