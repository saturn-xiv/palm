package models

import "gorm.io/gorm"

type Rule struct {
	gorm.Model

	Name    string `gorm:"uniqueIndex;not null;size:255"`
	Content []byte `gorm:"not null"`
	Version uint   `gorm:"not null;default:0"`
	Users   []User `gorm:"many2many:hosts_rules;"`
}

func (Rule) TableName() string {
	return "rules"
}
