package models

import (
	"bytes"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"encoding/gob"
	"encoding/hex"
	"fmt"
	"strings"
	"time"

	"github.com/go-playground/validator/v10"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
)

type EmailUser struct {
	ID        uint32 `gorm:"primaryKey"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt
	Version   uint32

	UserID      uint32
	RealName    string
	Email       string
	Password    string
	Avatar      string
	ConfirmedAt *time.Time
}

func (EmailUser) TableName() string {
	return "email_users"
}

func IsEmailUserExists(db *gorm.DB, email string) (bool, error) {
	var c int64
	if err := db.Model(&EmailUser{}).Where("email = ?", email).Count(&c).Error; err != nil {
		return false, err
	}
	return c > 0, nil
}

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

type CreateEmailUserForm struct {
	RealName string `validate:"required,min=2,max=63"`
	Email    string `validate:"required,email,max=127"`
	Password string `validate:"required,min=2,max=32"`
}

func (p *CreateEmailUserForm) Execute(db *gorm.DB, mac *crypto.HMac, lang string, timezone string) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	password, err := sign_password(mac, p.Password)
	if err != nil {
		return err
	}

	user_form := CreateUserForm{Lang: lang, Timezone: timezone}
	uid, err := user_form.Execute(db)
	if err != nil {
		return err
	}
	user, err := UserByUID(db, uid)
	if err != nil {
		return err
	}

	if err := db.Create(&EmailUser{
		UserID:   user.ID,
		Email:    p.Email,
		RealName: p.RealName,
		Password: password,
		Avatar:   Gravatar(p.Email),
	}).Error; err != nil {
		return err
	}
	return nil
}

type saltedPassword struct {
	Payload []byte
	Salt    []byte
}

func (p *saltedPassword) to_bytes() ([]byte, error) {
	var buf bytes.Buffer

	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func newSaltedPassword(s string) (*saltedPassword, error) {
	tmp, err := base64.RawURLEncoding.DecodeString(s)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	var it saltedPassword
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}
func sign_password(mac *crypto.HMac, plain string) (string, error) {
	salt := make([]byte, 8)
	if _, err := rand.Read(salt); err != nil {
		return "", err
	}
	code, err := sign_password_with_salt(mac, plain, salt)
	if err != nil {
		return "", err
	}
	return base64.RawURLEncoding.EncodeToString(code), nil
}
func sign_password_with_salt(mac *crypto.HMac, plain string, salt []byte) ([]byte, error) {
	pt := saltedPassword{Payload: []byte(plain), Salt: salt}
	pb, err := pt.to_bytes()
	if err != nil {
		return nil, err
	}
	code, err := mac.Sign(pb)
	if err != nil {
		return nil, err
	}
	ct := saltedPassword{Payload: code, Salt: salt}
	cb, err := ct.to_bytes()
	if err != nil {
		return nil, err
	}
	return cb, nil
}
func verify_password(mac *crypto.HMac, password string, plain string) (bool, error) {
	tmp, err := newSaltedPassword(password)
	if err != nil {
		return false, err
	}
	code, err := sign_password_with_salt(mac, plain, tmp.Salt)
	if err != nil {
		return false, err
	}
	return bytes.Equal(tmp.Payload, code), nil
}

func ConfirmEmailUser(db *gorm.DB, id uint32) error {
	now := time.Now()
	return db.Model(&EmailUser{}).Where("id = ?", id).Update("confirmed_at", now).Error
}

// https://docs.gravatar.com/avatars/go/
func Gravatar(email string) string {
	hasher := sha256.Sum256([]byte(strings.TrimSpace(strings.ToLower(email))))
	hash := hex.EncodeToString(hasher[:])
	return fmt.Sprintf("https://gravatar.com/avatar/%s", hash)
}

func UserByEmail(db *gorm.DB, email string) (*EmailUser, error) {
	var it EmailUser
	if err := db.Where("email = ?", email).First(&it).Error; err != nil {
		return nil, err
	}
	return &it, nil
}
