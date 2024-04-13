package models

import "time"

type LeaveWord struct {
	Model `gorm:"embedded"`

	Lang        string `gorm:"index;not null;size:15"`
	Ip          string `gorm:"index;not null;size:45"`
	Body        string `gorm:"index;not null"`
	Editor      string `gorm:"index;not null;size:15"`
	PublishedAt *time.Time
}
