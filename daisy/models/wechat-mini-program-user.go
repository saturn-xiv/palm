package models

type WechatMiniProgramUser struct {
	Model

	UserID    uint    `gorm:"not null"`
	UnionID   string  `gorm:"uniqueIndex;not null;size:127"`
	AppID     string  `gorm:"uniqueIndex:idx_wechat_mini_program_users_app_open_ids;index;not null;size:127"`
	OpenID    string  `gorm:"uniqueIndex:idx_wechat_mini_program_users_app_open_ids;index;not null;size:127"`
	Nickname  *string `gorm:"size:63"`
	AvatarUrl *string `gorm:"size:127"`

	User *User
}

func (WechatMiniProgramUser) TableName() string {
	return "wechat_mini_program_users"
}
