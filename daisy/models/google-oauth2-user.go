package models

import (
	"errors"
	"fmt"
	"time"

	"github.com/saturn-xiv/palm/daisy/env"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"golang.org/x/text/language"
	google_oauth2 "google.golang.org/api/oauth2/v2"
	"gorm.io/gorm"
)

type GoogleOauth2User struct {
	Model

	UserID        uint   `gorm:"not null"`
	Code          string `gorm:"uniqueIndex;not null;size:127"`
	Name          string `gorm:"index;not null;size:63"`
	Email         string `gorm:"uniqueIndex;not null;size:63"`
	EmailVerified *bool
	Picture       string `gorm:"not null;size:127"`
	Gender        string `gorm:"index;not null;size:15"`
	Link          string `gorm:"not null;size:127"`
	Locale        string `gorm:"index;not null;size:15"`

	User *User
}

func (GoogleOauth2User) TableName() string {
	return "google_oauth2_users"
}

func UserSignInByGoogleOauth2(db *gorm.DB, info *google_oauth2.Userinfo, ip string, lang *language.Tag, timezone *time.Location) error {
	var it GoogleOauth2User
	if err := db.Where("code = ?", info.Id).Preload("User").First(&it).Error; err != nil {
		return err
	}
	if it.User.LockedAt != nil {
		return fmt.Errorf("user %s is locked", info.Name)
	}
	if err := SignInUser(db, it.User, ip); err != nil {
		return err
	}
	if err := CreateLog(db, it.UserID, env.Plugin(), ip, v2.UserIndexLogResponse_Item_INFO, "sign in by google oauth2"); err != nil {
		return err
	}
	return nil
}
func userSignInByGoogleOauth2(db *gorm.DB, info *google_oauth2.Userinfo, lang *language.Tag, timezone *time.Location) error {
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
