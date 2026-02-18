package models

type WechatOauth2User struct {
	Model

	UserID     uint    `gorm:"not null"`
	UnionID    string  `gorm:"uniqueIndex;not null;size:127"`
	AppID      string  `gorm:"uniqueIndex:idx_wechat_oauth2_users_app_open_ids;index;not null;size:127"`
	OpenID     string  `gorm:"uniqueIndex:idx_wechat_oauth2_users_app_open_ids;indexlnot null;size:127"`
	Nickname   string  `gorm:"index;not null;size:63"`
	Sex        int     `gorm:"not null"`
	City       string  `gorm:"index;not null;size:63"`
	Province   string  `gorm:"index;not null;size:63"`
	Country    string  `gorm:"index;not null;size:63"`
	HeadImgUrl *string `gorm:"size:127"`
	Privilege  []byte  `gorm:"not null"`
	Lang       string  `gorm:"index;not null;size:7"`

	User *User
}

func (WechatOauth2User) TableName() string {
	return "wechat_oauth2_users"
}
