package models

import "time"

type Category struct {
	ID        uint32    `gorm:"primaryKey"`
	Code      string    `gorm:"uniqueIndex;not null;size:255"`
	Left      int32     `gorm:"not null"`
	Right     int32     `gorm:"not null"`
	SortOrder int32     `gorm:"not null"`
	Version   uint32    `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

type CategoryResource struct {
	ID         uint32    `gorm:"primaryKey"`
	CategoryID uint32    `gorm:"not null;index:,unique,composite:by_resource"`
	Resource   Resource  `gorm:"embedded"`
	SortOrder  int32     `gorm:"not null"`
	CreatedAt  time.Time `gorm:"not null"`
}
