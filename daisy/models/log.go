package models

import (
	"time"

	"gorm.io/gorm"

	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type Log struct {
	ID        uint      `gorm:"primarykey"`
	UserID    uint      `gorm:"not null"`
	Plugin    string    `gorm:"index;not null;size:15"`
	Ip        string    `gorm:"index;not null;size:45"`
	Level     int32     `gorm:"index;not null"`
	Message   string    `gorm:"not null;type:text"`
	CreatedAt time.Time `gorm:"not null"`

	User *User
}

func (Log) TableName() string {
	return "logs"
}

func CreateLog(db *gorm.DB, user uint, plugin string, ip string, level v2.UserIndexLogResponse_Item_Level, message string) error {
	return db.Create(&Log{
		UserID:  user,
		Plugin:  plugin,
		Ip:      ip,
		Level:   int32(level),
		Message: message,
	}).Error
}
