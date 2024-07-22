package models

import (
	"time"
)

type Model struct {
	ID        uint      `gorm:"primaryKey"`
	Version   uint      `gorm:"not null;default:0"`
	CreatedAt time.Time `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	DeletedAt *time.Time
}
