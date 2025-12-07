package models

import (
	"time"

	"gorm.io/gorm"
)

type Log struct {
	ID        uint
	UserID    uint   `gorm:"not null"`
	Plugin    string `gorm:"index;not null;size:15"`
	Ip        string `gorm:"index;not null;size:45"`
	Level     string `gorm:"index;not null;size:7"`
	Message   string `gorm:"not null;type:text"`
	CreatedAt time.Time

	User *User
}

func (Log) TableName() string {
	return "logs"
}

func CreateLog(db *gorm.DB, user uint, plugin string, ip string, level string, message string) error {
}
