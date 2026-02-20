package graphql

import (
	"context"
	"fmt"
	"reflect"
	"strings"
	"time"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	email_v2 "github.com/saturn-xiv/palm/daisy/email/v2"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/queue"
)

const (
	emailUserResetPasswordAudience = "email-user.reset-password"
	emailUserConfirmAudience       = "email-user.confirm"
	emailUserUnlockAudience        = "email-user.unlock"
)

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
	return newUserSignInResponse(p.db, v2.Session_EMAIL, form.Email, args.Ttl)
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
