package graphql

import (
	"crypto/hmac"
	"crypto/sha512"
	"encoding/base64"
	"errors"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/loquat/models"
)

type Administrator struct {
	Username string `validate:"required,gte=2,lte=15"`
	Password string `validate:"required,gte=6,lte=31"`
}

func (p *Administrator) Save(db *gorm.DB, key []byte) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	password := compute_password(p.Password, key)

	if err := db.Transaction(func(tx *gorm.DB) error {
		var user models.User
		err := db.Where(&models.User{Name: p.Username}, "name").Take(&user).Error
		if err == nil {
			if err = db.Model(&user).Updates(map[string]interface{}{
				"password": password,
				"version":  user.Version + 1,
			}).Error; err != nil {
				return err
			}
			if err = db.Create(&models.Log{UserID: user.ID, Message: "reset password"}).Error; err != nil {
				return err
			}
		} else if errors.Is(err, gorm.ErrRecordNotFound) {
			user.Name = p.Username
			user.Password = password
			if err = db.Create(&user).Error; err != nil {
				return err
			}
			if err = db.Create(&models.Log{UserID: user.ID, Message: "create account"}).Error; err != nil {
				return err
			}
		} else {
			return err
		}
		return nil
	}); err != nil {
		return err
	}

	return nil
}

func compute_password(str string, key []byte) string {
	mac := hmac.New(sha512.New, key)
	mac.Write([]byte(str))
	buf := mac.Sum(nil)
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf)
}
