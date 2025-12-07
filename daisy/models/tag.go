package models

import (
	"time"

	"gorm.io/gorm"
)

type Tag struct {
	gorm.Model

	Name    string `gorm:"uniqueIndex;not null;size:255"`
	Version uint   `gorm:"not null;default:0"`

	Resources []*TagResource
}

func (Tag) TableName() string {
	return "tags"
}

type TagResource struct {
	ID           uint   `gorm:"primarykey"`
	TagID        uint   `gorm:"not null"`
	ResourceType string `gorm:"index;not null;size:127"`
	ResourceId   uint
	CreatedAt    time.Time

	Tag *Tag
}

func (TagResource) TableName() string {
	return "tags_resources"
}
