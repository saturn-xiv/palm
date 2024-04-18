package models

import (
	"google.golang.org/protobuf/types/known/timestamppb"

	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type WechatMiniProgramUser struct {
	Model `gorm:"embedded"`

	UserID    uint32  `gorm:"uniqueIndex;not null"`
	UnionId   string  `gorm:"uniqueIndex;not null;size:127"`
	AppId     string  `gorm:"index;index:,unique,composite:app_open_ids;not null;size:63"`
	OpenId    string  `gorm:"index;index:,unique,composite:app_open_ids;not null;size:127"`
	Nickname  *string `gorm:"size:63"`
	AvatarUrl *string `gorm:"size:255"`
}

func (p *WechatMiniProgramUser) Detail() *pb.UserIndexResponse_Item_Detail {
	it := pb.UserIndexResponse_Item_Detail{
		ProviderType: pb.UserIndexResponse_Item_WeChatMiniProgram,
		Uid:          p.OpenId,
		ConfirmedAt:  timestamppb.New(p.Model.CreatedAt),
	}
	if p.AvatarUrl != nil {
		it.Avatar = *p.AvatarUrl
	}
	if p.Nickname != nil {
		it.Name = *p.Nickname
	}
	return &it
}
