package models

import (
	"time"
)

type Tag struct {
	Model

	Name string `gorm:"uniqueIndex;not null;size:63"`

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
	CreatedAt    time.Time `gorm:"not null"`

	Tag *Tag
}

func (TagResource) TableName() string {
	return "tags_resources"
}
