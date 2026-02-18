package models

type ShortenLink struct {
	Model

	Url   string `gorm:"uniqueIndex;not null;size:127"`
	Title string `gorm:"index;not null;size:63"`
	Memo  string `gorm:"index;not null;size:511"`
}

func (ShortenLink) TableName() string {
	return "shorten_link"
}
