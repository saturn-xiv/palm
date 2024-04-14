package models

import "time"

type EmailUser struct {
	Model `gorm:"embedded"`

	UserID      uint32 `gorm:"uniqueIndex;not null"`
	Nickname    string `gorm:"uniqueIndex;not null;size:31"`
	Email       string `gorm:"uniqueIndex;not null;size:127"`
	RealName    string `gorm:"index;not null;size:63"`
	Password    string `gorm:"not null;size:255"`
	Salt        string `gorm:"not null;size:32"`
	Avatar      string `gorm:"not null;size:255"`
	ConfirmedAt *time.Time
}
