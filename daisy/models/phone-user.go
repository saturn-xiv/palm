package models

import "time"

type PhoneUser struct {
	Model

	UserID      uint    `gorm:"not null"`
	Name        string  `gorm:"index;not null;size:63"`
	Phone       string  `gorm:"uniqueIndex;not null;size:15"`
	Password    string  `gorm:"not null;size:255"`
	Avatar      *string `gorm:"size:127"`
	ConfirmedAt *time.Time

	User *User
}

func (PhoneUser) TableName() string {
	return "phone_users"
}
