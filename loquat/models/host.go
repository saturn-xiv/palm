package models

import "gorm.io/gorm"

type Host struct {
	gorm.Model

	MemberID  *uint
	Member    *Member
	Name      string `gorm:"index;not null;size:63"`
	Mac       string `gorm:"index;not null;size:17"`
	Interface string `gorm:"index;not null;size:15"`
	Ip        string `gorm:"index;not null;size:39"`
	Fixed     bool   `gorm:"not null;default:false"`
	Version   uint   `gorm:"not null;default:0"`
	Rules     []Rule `gorm:"many2many:hosts_rules;"`
}

func (Host) TableName() string {
	return "hosts"
}
