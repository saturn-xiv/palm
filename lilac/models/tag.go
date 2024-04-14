package models

import "time"

type Tag struct {
	ID        uint32    `gorm:"primaryKey"`
	Code      string    `gorm:"uniqueIndex;not null;size:63"`
	SortOrder int32     `gorm:"not null"`
	Version   uint32    `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

type TagResource struct {
	ID        uint32    `gorm:"primaryKey"`
	TagID     uint32    `gorm:"not null;index:,unique,composite:by_resource"`
	Resource  Resource  `gorm:"embedded"`
	SortOrder int32     `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}
