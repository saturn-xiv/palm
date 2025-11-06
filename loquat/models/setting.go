package models

import "gorm.io/gorm"

type Setting struct {
	gorm.Model

	Key     string `gorm:"uniqueIndex;not null;size:255"`
	Value   string `gorm:"not null;type:text"`
	Version uint   `gorm:"not null;default:0"`
}

func (Setting) TableName() string {
	return "settings"
}
