package models

import (
	"time"

	"gorm.io/gorm"
)

type Attachment struct {
	gorm.Model

	UserID      uint   `gorm:"not null"`
	Title       string `gorm:"index;not null;size:127"`
	Bucket      string `gorm:"uniqueIndex:idx_bucket_object;not null;size:63"`
	Object      string `gorm:"uniqueIndex:idx_bucket_object;not null;size:63"`
	ContentType string `gorm:"index;not null;size:63"`
	Size        uint   `gorm:"not null"`
	Public      bool   `gorm:"not null;default:false"`
	Version     uint   `gorm:"not null;default:0"`

	Resources []*AttachmentResource
	User      *User
}

func (Attachment) TableName() string {
	return "attachments"
}

type AttachmentResource struct {
	ID           uint   `gorm:"primarykey"`
	AttachmentID uint   `gorm:"not null"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceId   uint
	CreatedAt    time.Time

	Attachment *Attachment
}

func (AttachmentResource) TableName() string {
	return "attachments_resources"
}
