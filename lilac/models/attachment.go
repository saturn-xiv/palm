package models

import "time"

type Attachment struct {
	Model `gorm:"embedded"`

	UserID      uint32 `gorm:"not null"`
	Bucket      string `gorm:"index;index:,unique,composite:bucket_name;not null;size:63"`
	Name        string `gorm:"index;index:,unique,composite:bucket_name;not null;size:63"`
	Title       string `gorm:"index;not null;size:127"`
	Size        int64  `gorm:"not null"`
	ContentType string `gorm:"index;not null;size:63"`
}

type AttachmentResource struct {
	ID           uint32    `gorm:"primaryKey"`
	AttachmentID uint32    `gorm:"not null;index:,unique,composite:by_resource"`
	Resource     Resource  `gorm:"embedded"`
	CreatedAt    time.Time `gorm:"not null"`
}
