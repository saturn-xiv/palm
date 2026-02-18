package models

type Vote struct {
	Model

	ResourceType string  `gorm:"index;not null;size:127"`
	ResourceId   uint    `gorm:"not null"`
	UserID       *uint   `gorm:"not null"`
	Value        int     `gorm:"not null"`
	Ip           string  `gorm:"index;not null;size:45"`
	Location     *string `gorm:"size:255"`

	User *User
}

func (Vote) TableName() string {
	return "votes"
}
