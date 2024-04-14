package models

import (
	"time"

	"gorm.io/gorm"
)

type Model struct {
	ID        uint32 `gorm:"primaryKey"`
	Version   uint32 `gorm:"not null"`
	DeletedAt *time.Time
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

type Resource struct {
	Type string `gorm:"column:resource_type;index;index:,unique,composite:by_resource;not null;size:127"`
	ID   uint32 `gorm:"column:resource_id;index:,unique,composite:by_resource;not null"`
}

func AutoMigrate(db *gorm.DB) error {
	return db.AutoMigrate(
		&User{}, &EmailUser{}, &GoogleUser{}, &WechatOauth2User{}, &WechatMiniProgramUser{},
		&Log{},
		&Session{},
		&Locale{},
		&Setting{},
		&Attachment{}, &AttachmentResource{},
		&Tag{}, &TagResource{},
		&Category{}, &CategoryResource{},
		&Notification{},
	)
}
