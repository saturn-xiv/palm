package models

import "time"

type Tag struct {
	ID        uint      `gorm:"primaryKey"`
	Code      string    `gorm:"uniqueIndex;not null;size:63"`
	SortOrder int       `gorm:"not null"`
	Version   uint      `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

type TagResource struct {
	ID        uint      `gorm:"primaryKey"`
	TagID     uint      `gorm:"not null;index:,unique,composite:by_resource"`
	Resource  Resource  `gorm:"embedded"`
	SortOrder int       `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}
