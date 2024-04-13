package models

import "time"

type Notification struct {
	Model `gorm:"embedded"`

	UserID  uint   `gorm:"not null"`
	Subject string `gorm:"not null;size:127"`
	Summary string `gorm:"not null;size:511"`
	Url     string `gorm:"not null;size:255"`
	ReadAt  *time.Time
}
