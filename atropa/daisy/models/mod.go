package models

import "time"

type TwilioSmsLogs struct {
	ID        uint
	Body      []byte `gorm:"not null"`
	CreatedAt time.Time
}
