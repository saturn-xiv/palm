package models

import (
	"time"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
)

type Setting struct {
	ID        uint32 `gorm:"primaryKey"`
	UserID    *uint32
	CreatedAt time.Time
	UpdatedAt time.Time
	Version   uint32

	Key   string
	Value []byte
	Nonce *[]byte
}

const (
	KeySiteDefaultLanguage = "site.default-language"
	KeySiteHomePage        = "site.homepage"
)

func (Setting) TableName() string {
	return "settings"
}

func Get(db *gorm.DB, aes *crypto.Aes, user *uint32, key string) ([]byte, error) {
	var it Setting
	if err := db.Where("user_id = ? AND key = ?", user, key).First(&it).Error; err != nil {
		return nil, err
	}
	if it.Nonce == nil {
		return it.Value, nil
	}
	return aes.Decrypt(it.Value, *it.Nonce)
}
func Set(db *gorm.DB, aes *crypto.Aes, user *uint32, key string, value []byte, encrypt bool) error {
	var it Setting
	err := db.Where("user_id = ? AND key = ?", user, key).First(&it).Error

	if encrypt {
		val, nonce, err := aes.Encrypt(value)
		if err != nil {
			return err
		}
		it.Value = val
		it.Nonce = &nonce
	} else {
		it.Value = value
		it.Nonce = nil
	}
	if err == nil {
		return db.Model(&it).Update("value", "nonce").Error
	}
	if err == gorm.ErrRecordNotFound {
		it.UserID = user
		it.Key = key
		return db.Save(&it).Error
	}
	return err
}
