package models

type SmsUser struct {
	Model

	UserID uint    `gorm:"not null"`
	Name   string  `gorm:"index;not null;size:63"`
	Phone  string  `gorm:"uniqueIndex;not null;size:15"`
	Avatar *string `gorm:"size:127"`

	User *User
}

func (SmsUser) TableName() string {
	return "sms_users"
}
