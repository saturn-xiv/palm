package models

import "time"

type Log struct {
	ID           uint   `gorm:"primaryKey"`
	UserID       uint   `gorm:"not null"`
	Plugin       string `gorm:"index;not null;size:31"`
	Level        string `gorm:"index;not null;size:8"`
	Ip           string `gorm:"index;not null;size:45"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceID   *uint
	Message      string    `gorm:"not null"`
	CreatedAt    time.Time `gorm:"not null"`
}
