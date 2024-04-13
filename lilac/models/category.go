package models

import "time"

type Category struct {
	ID        uint      `gorm:"primaryKey"`
	Code      string    `gorm:"uniqueIndex;not null;size:255"`
	Left      int       `gorm:"not null"`
	Right     int       `gorm:"not null"`
	SortOrder int       `gorm:"not null"`
	Version   uint      `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

type CategoryResource struct {
	ID         uint      `gorm:"primaryKey"`
	CategoryID uint      `gorm:"not null;index:,unique,composite:by_resource"`
	Resource   Resource  `gorm:"embedded"`
	SortOrder  int       `gorm:"not null"`
	CreatedAt  time.Time `gorm:"not null"`
}
