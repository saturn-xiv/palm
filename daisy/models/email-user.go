package models

import (
	"encoding/base64"
	"fmt"
	"strings"
	"time"

	"golang.org/x/text/language"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
)

type EmailUser struct {
	Model

	UserID      uint    `gorm:"not null"`
	Name        string  `gorm:"not null;size:63"`
	Email       string  `gorm:"uniqueIndex;not null;size:31"`
	Password    string  `gorm:"not null;size:255"`
	Avatar      *string `gorm:"size:127"`
	ConfirmedAt *time.Time
	Version     uint `gorm:"not null;default:0"`

	User *User
}

func (EmailUser) TableName() string {
	return "email_users"
}

func (p *EmailUser) VerifyPassword(hmac *crypto.Hmac, password string) error {
	tmp, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(p.Password)
	if err != nil {
		return err
	}
	return hmac.Verify(tmp, []byte(password))
}

type setPasswordForEmailUserForm struct {
	Password string `validate:"required,gte=6,lte=31"`
}

func NewSetPasswordForEmailUserForm(password string) *setPasswordForEmailUserForm {
	return &setPasswordForEmailUserForm{
		Password: password,
	}
}

func (p *setPasswordForEmailUserForm) Execute(db *gorm.DB, user *EmailUser, hmac *crypto.Hmac) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	password, err := compute_password(hmac, p.Password)
	if err != nil {
		return err
	}

	if err = db.Model(&user).Updates(map[string]interface{}{
		"password": password,
		"version":  user.Version + 1,
	}).Error; err != nil {
		return err
	}

	return nil
}

type resetPasswordForEmailUserForm struct {
	Email    string `validate:"required,gte=5,lte=31,email"`
	Password string `validate:"required,gte=6,lte=31"`
}

func NewResetPasswordForEmailUserForm(email string, password string) *resetPasswordForEmailUserForm {
	return &resetPasswordForEmailUserForm{
		Email:    ToCode(email),
		Password: password,
	}
}

func (p *resetPasswordForEmailUserForm) Execute(db *gorm.DB, hmac *crypto.Hmac) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	password, err := compute_password(hmac, p.Password)
	if err != nil {
		return err
	}

	var it EmailUser
	if err := db.Where("email = ?", p.Email).First(&it).Error; err != nil {
		return err
	}

	if err = db.Model(&it).Updates(map[string]interface{}{
		"password": password,
		"version":  it.Version + 1,
	}).Error; err != nil {
		return err
	}

	return nil
}

type createEmailByUserForm struct {
	Name     string `validate:"required,gte=2,lte=31"`
	Email    string `validate:"required,gte=5,lte=31,email"`
	Password string `validate:"required,gte=6,lte=31"`
}

func NewCreateEmailByUserForm(name string, email string, password string) *createEmailByUserForm {
	return &createEmailByUserForm{
		Name:     strings.TrimSpace(name),
		Email:    ToCode(email),
		Password: password,
	}
}

func (p *createEmailByUserForm) Execute(db *gorm.DB, hmac *crypto.Hmac, lang *language.Tag, timezone *time.Location) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	{
		var c int64
		if err := db.Model(&EmailUser{}).Where("email = ?", p.Email).Count(&c).Error; err != nil {
			return err
		}
		if c > 0 {
			return fmt.Errorf("user %s exists", p.Email)
		}
	}
	password, err := compute_password(hmac, p.Password)
	if err != nil {
		return err
	}

	user, err := CreateUser(db, lang, timezone)
	if err != nil {
		return err
	}

	if err := db.Create(&EmailUser{
		UserID:   user.ID,
		Name:     p.Name,
		Email:    p.Email,
		Password: password,
	}).Error; err != nil {
		return err
	}

	return nil
}

func ConfirmEmailUser(db *gorm.DB, id uint) error {
	var it EmailUser
	if err := db.First(&it, id).Error; err != nil {
		return err
	}
	now := time.Now()
	if err := db.Model(&it).Updates(map[string]interface{}{
		"confirmed_at": &now,
		"version":      it.Version + 1,
	}).Error; err != nil {
		return err
	}

	return nil

}

func compute_password(hmac *crypto.Hmac, password string) (string, error) {
	buf, err := hmac.Compute([]byte(password))
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
