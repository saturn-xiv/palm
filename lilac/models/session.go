package models

import "time"

type Session struct {
	ID           uint      `gorm:"primaryKey"`
	UserID       uint      `gorm:"not null"`
	Uid          string    `gorm:"uniqueIndex;not null;size:36"`
	ProviderType string    `gorm:"index;index:,unique,composite:provider;not null;size:31"`
	ProviderId   uint      `gorm:"index:,unique,composite:provider;not null"`
	Ip           string    `gorm:"index;not null;size:45"`
	ExpiredAt    time.Time `gorm:"not null"`
	CreatedAt    time.Time `gorm:"not null"`
}
