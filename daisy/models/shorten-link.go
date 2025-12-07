package models

import "gorm.io/gorm"

type ShortenLink struct {
	gorm.Model

	Url     string `gorm:"uniqueIndex;not null;size:127"`
	Title   string `gorm:"index;not null;size:63"`
	Memo    string `gorm:"index;not null;size:511"`
	Version uint   `gorm:"not null;default:0"`
}

func (ShortenLink) TableName() string {
	return "shorten_link"
}
