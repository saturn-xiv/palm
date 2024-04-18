package models

import "time"

type Log struct {
	ID           uint32 `gorm:"primaryKey"`
	UserID       uint32 `gorm:"not null"`
	Plugin       string `gorm:"index;not null;size:31"`
	Level        int32  `gorm:"not null"`
	Ip           string `gorm:"index;not null;size:45"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceID   *uint32
	Message      string    `gorm:"not null"`
	CreatedAt    time.Time `gorm:"not null"`
}
