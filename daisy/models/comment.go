package models

type Comment struct {
	Model

	ResourceType string `gorm:"index;not null;size:127"`
	ResourceId   uint   `gorm:"not null"`
	CommentID    *uint
	UserID       *uint
	Body         string  `gorm:"not null;type:text"`
	Editor       int32   `gorm:"index;not null"`
	Ip           string  `gorm:"index;not null;size:45"`
	Location     *string `gorm:"size:255"`

	Comment *Comment
	User    *User
}

func (Comment) TableName() string {
	return "comments"
}
