package models

import (
	"errors"
	"time"

	"github.com/google/uuid"
	"golang.org/x/text/language"
	google_oauth2 "google.golang.org/api/oauth2/v2"
	"gorm.io/gorm"
)

type GoogleOauth2User struct {
	gorm.Model

	UserID        uint   `gorm:"not null"`
	Sn            string `gorm:"uniqueIndex;not null;size:36"`
	Code          string `gorm:"uniqueIndex;not null;size:127"`
	Name          string `gorm:"not null;size:63"`
	Email         string `gorm:"not null;size:63"`
	EmailVerified *bool
	Picture       string `gorm:"not null;size:127"`
	Gender        string `gorm:"not null;size:15"`
	Link          string `gorm:"not null;size:127"`
	Locale        string `gorm:"not null;size:15"`
	Version       uint   `gorm:"not null;default:0"`

	User *User
}

func (GoogleOauth2User) TableName() string {
	return "google_oauth2_users"
}

func UserSignInByGoogleOauth2(db *gorm.DB, info *google_oauth2.Userinfo, lang *language.Tag, timezone *time.Location) error {
	var it GoogleOauth2User
	err := db.Where("code = ?", info.Id).First(&it).Error
	if err == nil {
		if err = db.Model(&it).Updates(map[string]interface{}{
			"name":           info.Name,
			"email":          info.Email,
			"email_verified": info.VerifiedEmail,
			"picture":        info.Picture,
			"gender":         info.Gender,
			"link":           info.Link,
			"locale":         info.Locale,
			"version":        it.Version + 1,
		}).Error; err != nil {
			return err
		}
		return nil
	}
	if !errors.Is(err, gorm.ErrRecordNotFound) {
		return err
	}
	user, err := CreateUser(db, lang, timezone)
	if err != nil {
		return err
	}
	it.Code = info.Id
	it.Sn = uuid.New().String()
	it.UserID = user.ID
	it.Name = info.Name
	it.Email = info.Email
	it.EmailVerified = info.VerifiedEmail
	it.Picture = info.Picture
	it.Gender = info.Gender
	it.Link = info.Link
	it.Locale = info.Locale
	if err := db.Create(&it).Error; err != nil {
		return err
	}

	return nil
}
