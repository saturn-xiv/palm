package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type User struct {
	gorm.Model

	Sn       string `gorm:"uniqueIndex;not null;size:36"`
	Lang     string `gorm:"index;not null;size:15;default:'en-US'"`
	Timezone string `gorm:"index;not null;size:31;default:'UTC'"`
	LockedAt *time.Time
	Version  uint `gorm:"not null;default:0"`

	Logs []Log
}

func (User) TableName() string {
	return "users"
}

func createUser(db *gorm.DB) (*User, error) {
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
