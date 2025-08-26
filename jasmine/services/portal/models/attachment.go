package models

import (
	"time"

	"gorm.io/gorm"
)

type Attachment struct {
	ID        uint32 `gorm:"primaryKey"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt
	Version   uint32

	UserID      uint32
	Bucket      string
	Object      string
	Title       string
	Size        uint32
	ContentType string
	UploadedAt  *time.Time
}

func (Attachment) TableName() string {
	return "attachments"
}
