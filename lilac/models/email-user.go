package models

import (
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"time"

	"google.golang.org/protobuf/types/known/timestamppb"
	"gorm.io/gorm"

	auth_pb "github.com/saturn-xiv/palm/lilac/auth/v2"
	"github.com/saturn-xiv/palm/lilac/env/crypto"
	rbac_pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

type EmailUser struct {
	Model `gorm:"embedded"`

	UserID      uint32 `gorm:"uniqueIndex;not null"`
	Nickname    string `gorm:"uniqueIndex;not null;size:31"`
	Email       string `gorm:"uniqueIndex;not null;size:127"`
	RealName    string `gorm:"index;not null;size:63"`
	Password    string `gorm:"not null;size:255"`
	Salt        string `gorm:"not null;size:63"`
	Avatar      string `gorm:"not null;size:255"`
	ConfirmedAt *time.Time
}

func (p *EmailUser) Detail() *auth_pb.UserIndexResponse_Item_Detail {
	it := auth_pb.UserIndexResponse_Item_Detail{
		ProviderType: rbac_pb.UserDetail_Provider_Email,
		ProviderId:   p.Nickname,
		Name:         p.RealName,
		Avatar:       p.Avatar,
	}
	if p.ConfirmedAt != nil {
		it.ConfirmedAt = timestamppb.New(*p.ConfirmedAt)
	}
	return &it
}

func ComputePassword(mac *crypto.HMac, plain string, salt_len uint8) (string, string, error) {
	salt := make([]byte, salt_len)
	if _, err := rand.Read(salt); err != nil {
		return "", "", err
	}
	var buf []byte
	buf = append(buf, []byte(plain)...)
	buf = append(buf, salt...)
	passwd, err := mac.Sign(buf)
	if err != nil {
		return "", "", err
	}

	enc := base64.StdEncoding.WithPadding(base64.NoPadding)
	return enc.EncodeToString(passwd), enc.EncodeToString(salt), nil
}

func VerifyPassword(mac *crypto.HMac, plain string, code string, salt string) error {
	dec := base64.StdEncoding.WithPadding(base64.NoPadding)
	salt_b, err := dec.DecodeString(salt)
	if err != nil {
		return err
	}
	code_b, err := dec.DecodeString(code)
	if err != nil {
		return err
	}
	var buf []byte
	buf = append(buf, []byte(plain)...)
	buf = append(buf, salt_b...)
	return mac.Verify(code_b, buf)
}
func CreateUserByEmail(db *gorm.DB, mac *crypto.HMac, real_name string, nickname string, email string, password string, lang string, timezone string) error {
	{
		var c int64
		if rst := db.Where(&EmailUser{Nickname: nickname}).Count(&c); rst.Error != nil {
			return rst.Error
		}
		if c > 0 {
			return fmt.Errorf("nickname %s already exists", nickname)
		}
	}
	{
		var c int64
		if rst := db.Where(&EmailUser{Email: email}).Count(&c); rst.Error != nil {
			return rst.Error
		}
		if c > 0 {
			return fmt.Errorf("email %s already exists", email)
		}
	}
	passwd, salt, err := ComputePassword(mac, password, 8)
	if err != nil {
		return err
	}
	now := time.Now()
	user := User{
		Lang:     lang,
		Timezone: timezone,
	}
	if rst := db.Create(&user); rst.Error != nil {
		return rst.Error
	}

	eu := EmailUser{
		UserID:   user.ID,
		Nickname: nickname,
		RealName: real_name,
		Email:    email,
		Password: passwd,
		Salt:     salt,
		Model: Model{
			UpdatedAt: now,
			CreatedAt: now,
		},
	}
	if rst := db.Create(&eu); rst.Error != nil {
		return rst.Error
	}
	if rst := db.Create(&Log{
		UserID:    user.ID,
		Level:     int32(auth_pb.UserLogsResponse_Item_Info),
		Message:   "Sign up",
		CreatedAt: now,
	}); rst.Error != nil {
		return rst.Error
	}
	return nil
}
