package models

type Setting struct {
	Model `gorm:"embedded"`

	UserID uint
	Key    string `gorm:"index;not null;size:255"`
	Salt   string `gorm:"size:32"`
	Value  []byte `gorm:"not null"`
}
