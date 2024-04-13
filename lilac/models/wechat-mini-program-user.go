package models

type WechatMiniProgramUser struct {
	Model `gorm:"embedded"`

	UserID    uint    `gorm:"uniqueIndex;not null"`
	UnionId   string  `gorm:"uniqueIndex;not null;size:127"`
	AppId     string  `gorm:"index;index:,unique,composite:app_open_ids;not null;size:63"`
	OpenId    string  `gorm:"index;index:,unique,composite:app_open_ids;not null;size:127"`
	Nickname  *string `gorm:"size:63"`
	AvatarUrl *string `gorm:"size:255"`
}
