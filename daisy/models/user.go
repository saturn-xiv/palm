package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type User struct {
	gorm.Model

	Sn                string `gorm:"uniqueIndex;not null;size:36"`
	Lang              string `gorm:"index;not null;size:15;default:'en-US'"`
	Timezone          string `gorm:"index;not null;size:31;default:'UTC'"`
	SignedInTotal     uint   `gorm:"not null;default:0"`
	CurrentSignedInAt *time.Time
	CurrentSignedInIp *string `gorm:"size:45"`
	LastSignedInAt    *time.Time
	LastSignedInIp    *string `gorm:"size:45"`
	LockedAt          *time.Time
	Version           uint `gorm:"not null;default:0"`

	Logs []Log
}

func (User) TableName() string {
	return "users"
}

func CreateUser(db *gorm.DB) (*User, error) {
	sn := uuid.New().String()
	if err := db.Create(&User{Sn: sn}).Error; err != nil {
		return nil, err
	}
	var it User
	if err := db.Where("sn = ?", sn).First(&it).Error; err != nil {
		return nil, err
	}
	return &it, nil
}

func SignInUser(db *gorm.DB, user *User, ip string) error {
	now := time.Now()

	if err := db.Model(&user).Updates(map[string]interface{}{
		"current_signed_in_at": &now,
		"current_signed_in_ip": ip,
		"last_signed_in_at":    user.CurrentSignedInAt,
		"last_signed_in_ip":    user.CurrentSignedInIp,
		"signed_in_total":      user.SignedInTotal + 1,
		"version":              user.Version + 1,
	}).Error; err != nil {
		return err
	}
	return nil
}
