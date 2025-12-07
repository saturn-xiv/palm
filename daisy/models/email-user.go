package models

import (
	"time"

	"gorm.io/gorm"
)

type EmailUser struct {
	gorm.Model

	UserID      uint    `gorm:"not null"`
	Sn          string  `gorm:"uniqueIndex;not null;size:36"`
	Name        string  `gorm:"not null;size:63"`
	Email       string  `gorm:"uniqueIndex;not null;size:31"`
	Password    string  `gorm:"not null;size:255"`
	Avatar      *string `gorm:"not null;size:127"`
	ConfirmedAt *time.Time
	Version     uint `gorm:"not null;default:0"`

	User *User
}

func (EmailUser) TableName() string {
	return "email_users"
}
