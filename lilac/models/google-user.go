package models

import (
	"google.golang.org/protobuf/types/known/timestamppb"

	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

// https://developers.google.com/identity/protocols/oauth2
// https://developers.google.com/identity/protocols/oauth2/web-server
// https://github.com/googleapis/google-api-go-client/tree/main
type GoogleUser struct {
	Model `gorm:"embedded"`

	UserID     uint32 `gorm:"uniqueIndex;not null"`
	Email      string `gorm:"uniqueIndex;not null;size:127"`
	Name       string `gorm:"index;not null;size:63"`
	GivenName  string `gorm:"index;not null;size:31"`
	FamilyName string `gorm:"index;not null;size:31"`
	Gender     string `gorm:"index;not null;size:15"`
	Picture    string `gorm:"not null;size:255"`
	Locale     string `gorm:"index;not null;size:15"`
	Sub        string `gorm:"uniqueIndex;not null;size:127"`
	Code       string `gorm:"not null"`
	Token      string `gorm:"not null;size:127"`
}

func (p *GoogleUser) Detail() *pb.UserIndexResponse_Item_Detail {
	it := pb.UserIndexResponse_Item_Detail{
		ProviderType: pb.UserIndexResponse_Item_Google,
		Uid:          p.Sub,
		Name:         p.Name,
		Avatar:       p.Picture,
		ConfirmedAt: timestamppb.New(p.Model.CreatedAt),
	}

	return &it
}
