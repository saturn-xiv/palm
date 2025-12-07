package models

import (
	"errors"

	"gorm.io/gorm"

	google_oauth2 "google.golang.org/api/oauth2/v2"
)

type GoogleOauth2User struct {
	gorm.Model

	UserID        uint   `gorm:"not null"`
	Sn            string `gorm:"uniqueIndex;not null;size:127"`
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

func SaveGoogleOauth2User(db *gorm.DB, info *google_oauth2.Userinfo) error {
	var it GoogleOauth2User
	err := db.Where("sn = ?", info.Id).First(&it).Error
	if err == nil {
		return db.Model(&it).Updates(map[string]interface{}{
			"name":           info.Name,
			"email":          info.Email,
			"email_verified": info.VerifiedEmail,
			"picture":        info.Picture,
			"gender":         info.Gender,
			"link":           info.Link,
			"locale":         info.Locale,
		}).Error
	}
	if !errors.Is(err, gorm.ErrRecordNotFound) {
		return err
	}
	user, err := createUser(db)
	if err != nil {
		return err
	}
	it.Sn = info.Id
	it.UserID = user.ID
	it.Name = info.Name
	it.Email = info.Email
	it.EmailVerified = info.VerifiedEmail
	it.Picture = info.Picture
	it.Gender = info.Gender
	it.Link = info.Link
	it.Locale = info.Locale
	return db.Create(&it).Error
}
