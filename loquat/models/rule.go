package models

import "gorm.io/gorm"

type Rule struct {
	gorm.Model

	Name      string   `gorm:"uniqueIndex;not null;size:255"`
	Type      string   `gorm:"index;not null;size:63"`
	Content   []byte   `gorm:"not null"`
	SortOrder int      `gorm:"not null;default:0"`
	Memo      string   `gorm:"not null;type:text"`
	Version   uint     `gorm:"not null;default:0"`
	Members   []Member `gorm:"many2many:members_rules;"`
}

func (Rule) TableName() string {
	return "rules"
}
