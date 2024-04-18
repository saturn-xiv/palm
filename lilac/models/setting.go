package models

import (
	"encoding/base64"
	"errors"
	"time"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
)

type Setting struct {
	Model `gorm:"embedded"`

	UserID *uint32
	Key    string  `gorm:"index;not null;size:255"`
	Salt   *string `gorm:"size:255"`
	Value  []byte  `gorm:"not null"`
}

func Get(db *gorm.DB, aes *crypto.Aes, user uint32, key string) ([]byte, error) {
	var it Setting
	if rst := db.Where(&Setting{Key: key, UserID: &user}).First(&it); rst.Error != nil {
		return nil, rst.Error
	}
	if it.Salt == nil {
		return it.Value, nil
	}
	salt, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(*it.Salt)
	if err != nil {
		return nil, err
	}
	return aes.Decrypt(it.Value, salt)
}
func Set(db *gorm.DB, aes *crypto.Aes, user uint32, key string, value []byte, encrypt bool) error {
	now := time.Now()
	var it Setting
	rst := db.Where(&Setting{Key: key, UserID: &user}).First(&it)
	if rst.Error == nil {
		if encrypt {
			val, salt, err := aes.Encrypt(value)
			if err != nil {
				return err
			}
			salt_s := base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(salt)
			if rst := db.Create(&Setting{
				Key:    key,
				UserID: &user,
				Value:  val,
				Salt:   &salt_s,
				Model:  Model{UpdatedAt: now, CreatedAt: now},
			}); rst.Error != nil {
				return rst.Error
			}
		} else {
			if rst := db.Create(&Setting{
				Key:    key,
				UserID: &user,
				Value:  value,
				Model:  Model{UpdatedAt: now, CreatedAt: now},
			}); rst.Error != nil {
				return rst.Error
			}
		}
	} else if errors.Is(rst.Error, gorm.ErrRecordNotFound) {
		if encrypt {
			val, salt, err := aes.Encrypt(value)
			if err != nil {
				return err
			}
			salt_s := base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(salt)
			it.Salt = &salt_s
			it.Value = val
			it.UpdatedAt = now
			it.Version += 1
			if rst := db.Save(&it); rst.Error != nil {
				return rst.Error
			}
		} else {
			it.Value = value
			it.UpdatedAt = now
			it.Version += 1
			if rst := db.Save(&it); rst.Error != nil {
				return rst.Error
			}
		}
	} else {
		return rst.Error
	}
	return nil
}

func Get_(db *gorm.DB, aes *crypto.Aes, key string) ([]byte, error) {
	var it Setting
	if rst := db.Where(&Setting{Key: key}).First(&it); rst.Error != nil {
		return nil, rst.Error
	}
	if it.Salt == nil {
		return it.Value, nil
	}
	salt, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(*it.Salt)
	if err != nil {
		return nil, err
	}
	return aes.Decrypt(it.Value, salt)
}
func Set_(db *gorm.DB, aes *crypto.Aes, key string, value []byte, encrypt bool) error {
	now := time.Now()
	var it Setting
	rst := db.Where(&Setting{Key: key}).First(&it)
	if rst.Error == nil {
		if encrypt {
			val, salt, err := aes.Encrypt(value)
			if err != nil {
				return err
			}
			salt_s := base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(salt)
			if rst := db.Create(&Setting{
				Key:   key,
				Value: val,
				Salt:  &salt_s,
				Model: Model{UpdatedAt: now, CreatedAt: now},
			}); rst.Error != nil {
				return rst.Error
			}
		} else {
			if rst := db.Create(&Setting{
				Key:   key,
				Value: value,
				Model: Model{UpdatedAt: now, CreatedAt: now},
			}); rst.Error != nil {
				return rst.Error
			}
		}
	} else if errors.Is(rst.Error, gorm.ErrRecordNotFound) {
		if encrypt {
			val, salt, err := aes.Encrypt(value)
			if err != nil {
				return err
			}
			salt_s := base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(salt)
			it.Salt = &salt_s
			it.Value = val
			it.UpdatedAt = now
			it.Version += 1
			if rst := db.Save(&it); rst.Error != nil {
				return rst.Error
			}
		} else {
			it.Value = value
			it.UpdatedAt = now
			it.Version += 1
			if rst := db.Save(&it); rst.Error != nil {
				return rst.Error
			}
		}
	} else {
		return rst.Error
	}
	return nil
}
