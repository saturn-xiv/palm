package models

import "time"

type User struct {
	Model `gorm:"embedded"`

	Lang              string `gorm:"index:,not null;size:15"`
	Timezone          string `gorm:"index:,not null;size:31"`
	SignInCount       uint   `gorm:"not null"`
	CurrentSignedInAt *time.Time
	CurrentSignedInIp *string `gorm:"size:45"`
	LastSignedInAt    *time.Time
	LastSignedInIp    *string `gorm:"size:45"`
	LockedAt          *time.Time
}
