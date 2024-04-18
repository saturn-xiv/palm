package models

import (
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
	"google.golang.org/protobuf/types/known/timestamppb"
)

type WechatOauth2User struct {
	Model `gorm:"embedded"`

	UserID     uint32   `gorm:"uniqueIndex;;not null"`
	UnionId    string   `gorm:"uniqueIndex;not null;size:127"`
	AppId      string   `gorm:"index;index:,unique,composite:app_open_ids;not null;size:63"`
	OpenId     string   `gorm:"index;index:,unique,composite:app_open_ids;not null;size:127"`
	Nickname   string   `gorm:"index;not null;size:63"`
	Sex        uint32   `gorm:"not null"`
	City       string   `gorm:"index;not null;size:63"`
	Province   string   `gorm:"index;not null;size:63"`
	Country    string   `gorm:"index;not null;size:63"`
	HeadImgUrl *string  `gorm:"size:255"`
	Privilege  []string `gorm:"not null;serializer:json"`
	Lang       string   `gorm:"index;not null;size:15"`
}

func (p *WechatOauth2User) Detail() *pb.UserIndexResponse_Item_Detail {
	it := pb.UserIndexResponse_Item_Detail{
		ProviderType: pb.UserIndexResponse_Item_WeChatOauth,
		Uid:          p.OpenId,
		Name:         p.Nickname,
		ConfirmedAt:  timestamppb.New(p.Model.CreatedAt),
	}
	if p.HeadImgUrl != nil {
		it.Avatar = *p.HeadImgUrl
	}

	return &it
}
