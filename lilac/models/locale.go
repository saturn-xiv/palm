package models

type Locale struct {
	Model `gorm:"embedded"`

	Lang    string `gorm:"index;index:,unique,composite:lang_code;not null;size:15"`
	Code    string `gorm:"index;index:,unique,composite:lang_code;not null;size:255"`
	Message string `gorm:"not null"`
}
