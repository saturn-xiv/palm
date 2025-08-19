package models

import (
	"time"

	"github.com/google/uuid"
	"golang.org/x/text/language"
	"gorm.io/gorm"
)

const (
	EN_US = "en-US"
	UTC   = "UTC"
)

type User struct {
	ID        uint32 `gorm:"primaryKey"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt
	Version   uint32

	UID             string
	Lang            string
	Timezone        string
	SignInCount     uint32
	CurrentSignInAt *time.Time
	CurrentSignInIP *string
	LastSignInAt    *time.Time
	LastSignInIP    *string
	LockedAt        *time.Time
}

func (User) TableName() string {
	return "users"
}

type CreateUserForm struct {
	Lang     string `validate:"required,min=2,max=15"`
	Timezone string `validate:"required,min=3,max=31"`
}

func (p *CreateUserForm) Execute(db *gorm.DB) (string, error) {
	if err := gl_validate.Struct(p); err != nil {
		return "", err
	}
	lang, err := language.Parse(p.Lang)
	if err != nil {
		return "", err
	}
	tz, err := time.LoadLocation(p.Timezone)
	if err != nil {
		return "", err
	}

	uid := uuid.New().String()
	if err := db.Create(&User{UID: uid, Lang: lang.String(), Timezone: tz.String()}).Error; err != nil {
		return "", err
	}
	return uid, nil
}

func UserByUID(db *gorm.DB, uid string) (*User, error) {
	var it User
	if err := db.Where("uid = ?", uid).First(&it).Error; err != nil {
		return nil, err
	}
	return &it, nil
}
