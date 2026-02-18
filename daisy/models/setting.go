package models

import (
	"bytes"
	"crypto/rand"
	"encoding/gob"
	"errors"

	"google.golang.org/protobuf/proto"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
)

type Setting struct {
	Model

	UserID *uint
	Key    string `gorm:"index;not null;size:255"`
	Value  []byte `gorm:"not null;type:bytes"`
	Salt   []byte `gorm:"type:bytes"`
}

func (Setting) TableName() string {
	return "settings"
}

func set(db *gorm.DB, key string, value []byte, salt []byte) error {
	var it Setting
	err := db.Where(map[string]interface{}{"key": key}).Take(&it).Error

	if err == nil {
		return db.Model(&it).Updates(map[string]interface{}{
			"value":   value,
			"salt":    salt,
			"version": it.Version + 1,
		}).Error

	}
	if !errors.Is(err, gorm.ErrRecordNotFound) {
		return err
	}
	it.Key = key
	it.Salt = salt
	it.Value = value
	return db.Create(&it).Error
}

func Set(db *gorm.DB, aead *crypto.Aead, key string, value []byte, encrypt bool) error {
	if encrypt {
		salt := make([]byte, 32)
		if _, err := rand.Read(salt); err != nil {
			return err
		}
		buf, err := aead.Encrypt(value, salt)
		if err != nil {
			return err
		}
		return set(db, key, buf, salt)
	}
	return set(db, key, value, nil)
}

func Get(db *gorm.DB, aead *crypto.Aead, key string) ([]byte, error) {
	var it Setting
	if err := db.Where(map[string]interface{}{"key": key}).Take(&it).Error; err != nil {
		return nil, err
	}
	if it.Salt != nil {
		return aead.Decrypt(it.Value, it.Salt)
	}
	return it.Value, nil
}

func SetProtobuf(db *gorm.DB, aead *crypto.Aead, key string, val proto.Message, encrypt bool) error {
	buf, err := proto.Marshal(val)
	if err != nil {
		return err
	}
	return Set(db, aead, key, buf, encrypt)
}

func GetProtobuf(db *gorm.DB, aead *crypto.Aead, key string, val proto.Message) error {
	buf, err := Get(db, aead, key)
	if err != nil {
		return err
	}
	return proto.Unmarshal(buf, val)
}

func SetB(db *gorm.DB, aead *crypto.Aead, key string, val interface{}, encrypt bool) error {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(val); err != nil {
		return err
	}
	return Set(db, aead, key, buf.Bytes(), encrypt)
}

func GetB(db *gorm.DB, aead *crypto.Aead, key string, val interface{}) error {
	tmp, err := Get(db, aead, key)
	if err != nil {
		return err
	}
	buf := bytes.NewBuffer(tmp)
	dec := gob.NewDecoder(buf)
	return dec.Decode(val)
}
