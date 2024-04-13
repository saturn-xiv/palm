package models

type WechatOauth2User struct {
	Model `gorm:"embedded"`

	UserID     uint     `gorm:"uniqueIndex;;not null"`
	UnionId    string   `gorm:"uniqueIndex;not null;size:127"`
	AppId      string   `gorm:"index;index:,unique,composite:app_open_ids;not null;size:63"`
	OpenId     string   `gorm:"index;index:,unique,composite:app_open_ids;not null;size:127"`
	Nickname   string   `gorm:"index;not null;size:63"`
	Sex        uint8    `gorm:"not null"`
	City       string   `gorm:"index;not null;size:63"`
	Province   string   `gorm:"index;not null;size:63"`
	Country    string   `gorm:"index;not null;size:63"`
	HeadImgUrl *string  `gorm:"size:255"`
	Privilege  []string `gorm:"not null;serializer:json"`
	Lang       string   `gorm:"index;not null;size:15"`
}
